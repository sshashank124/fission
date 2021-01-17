use std::collections::HashMap;
use std::mem;

use bumpalo::Bump;
#[allow(clippy::wildcard_imports)]
use graphite::*;

use crate::shape::{Intersectable, intersection::Its};
use crate::util::either::Either;

const MAX_LEAF_LEN: I = 4;
const NUM_BUCKETS: usize = 24;

#[derive(Debug)]
pub struct BVH<S> {
        nodes:    Box<[BVHNode]>,
    pub elements: Box<[S]>,
}

#[derive(Debug)]
pub struct BVHNode {
    pub bbox: BBox,
        node: BVHNodeType,
}

#[derive(Debug)]
pub enum BVHNodeType {
    Leaf(I, i16),
    Tree(I, Dim),
}

impl<S> BVH<S> where S: Intersectable
{
    pub fn new(elems: Vec<S>) -> Self {
        assert!(!elems.is_empty());

        let mut build_infos = elems.iter().enumerate().map(|(idx, e)| {
            let bbox = e.bbox();
            BuildInfo { bbox, center: bbox.center(), idx: idx.conv(),
                        isect_cost: e.intersection_cost() }
        }).collect::<Vec<_>>();

        let mut idx_map = HashMap::with_capacity(elems.len());
        let arena = Bump::with_capacity(elems.len() * mem::size_of::<BuildNode>());
        let root = build(&mut build_infos[..], &mut idx_map, &arena);

        let mut nodes = Vec::with_capacity(root.size().conv());
        flatten_tree(root, &mut nodes, 0);

        let idx_ord = (0..elems.len().conv()).map(|i| idx_map[&i]);
        let mut elems_idx = elems.into_iter().zip(idx_ord).collect::<Vec<_>>();
        elems_idx.sort_unstable_by_key(|(_, i)| *i);
        let elements = elems_idx.into_iter().map(|(e, _)| e)
                                .collect::<Vec<_>>().into_boxed_slice();

        Self { elements, nodes: nodes.into_boxed_slice() }
    }

    #[inline] pub fn fold<'a, A>(&'a self,
                       trav_order: A3<bool>,
                       mut acc: A,
                       pred: impl Fn(&mut A, &BVHNode) -> bool,
                       f: impl Fn(A, usize, &'a S) -> Either<A, A>)
                       -> A {
        let mut idx = 0;
        let mut stack: [I; 32] = [0; 32];
        let mut sp = 0;
        loop {
            let node = &self.nodes[usize::of(idx)];
            if pred(&mut acc, node) {
                match node.node {
                    BVHNodeType::Tree(ri, dim) => {
                        let ii = A2(ri, idx + 1);
                        stack[sp] = ii[!trav_order[dim]];
                        sp += 1;
                        idx = ii[trav_order[dim]];
                    }
                    BVHNodeType::Leaf(i, n) => {
                        for j in usize::of(i)..usize::of(i + I::of(n)) {
                            acc = match f(acc, j, &self.elements[j]) {
                                Either::L(b) => return b,
                                Either::R(a) => a,
                            };
                        }
                        idx = if sp == 0 { break } else {
                            sp -= 1;
                            stack[sp]
                        };
                    }
                }
            } else {
                idx = if sp == 0 { break } else {
                    sp -= 1;
                    stack[sp]
                };
            }
        }
        acc
    }
}

struct BuildNode<'a> {
    bbox:  BBox,
    node:  BuildNodeType<'a>,
    sizel: I,
    sizer: I,
}

enum BuildNodeType<'a> {
    Leaf(I, I),
    Tree(Dim, &'a BuildNode<'a>, &'a BuildNode<'a>),
}

struct BuildInfo {
    bbox:       BBox,
    center:     P,
    idx:        I,
    isect_cost: F,
}

impl BuildNode<'_> { const fn size(&self) -> I { self.sizel + self.sizer + 1 } }

fn flatten_tree(tree: &BuildNode, nodes: &mut Vec<BVHNode>, mut offset: I) {
    offset += 1;
    let node = match tree.node {
        BuildNodeType::Leaf(idx, n) => BVHNodeType::Leaf(idx, i16::of(n)),
        BuildNodeType::Tree(dim, treel, _) => {
            BVHNodeType::Tree(offset + treel.size(), dim)
        }
    };

    nodes.push(BVHNode { bbox: tree.bbox, node });

    if let BuildNodeType::Tree(_, treel, treer) = &tree.node {
        flatten_tree(treel, nodes, offset);
        flatten_tree(treer, nodes, offset + treel.size());
    }
}

#[derive(Clone, Copy)]
struct Bucket {
    cost: F,
    bbox: BBox,
}

fn build<'a>(build_infos: &mut [BuildInfo], idx_map: &mut HashMap<I, I>,
             arena: &'a Bump) -> &'a BuildNode<'a> {
    let n = build_infos.len().conv();

    if n <= MAX_LEAF_LEN {
        build_infos.iter().for_each(|bi| {
                              idx_map.insert(bi.idx, idx_map.len().conv());
                          });
        return arena.alloc(BuildNode {
            bbox: build_infos.iter().fold(BBox::ZERO, |bb, bi| bb | bi.bbox),
            node: BuildNodeType::Leaf(I::of(idx_map.len()) - n, n),
            sizel: 0, sizer: 0
        })
    }

    let (bbox, centers_bbox) =
        build_infos.iter().fold((BBox::ZERO, BBox::ZERO), |(bb, bc), b| {
                              (bb | b.bbox, bc | b.center)
                          });

    let (extent, dim) = centers_bbox.max_extent();

    let (dim, pivot) = if F::abs(extent) < F::EPS { (dim, n / 2) } else {
        let bucket_index = |build_info: &BuildInfo, dim: Dim| {
            let idx = I::of(F::of(NUM_BUCKETS)
                                   * ((build_info.center[dim]
                                       - centers_bbox[dim][0])
                                      / centers_bbox.extents()[dim]));
            idx.min(I::of(NUM_BUCKETS) - 1)
        };

        let (_, mc_idx, mc_dim) = XYZ.map(|dim| {
            let mut buckets =
                [Bucket { cost: 0., bbox: BBox::ZERO }; NUM_BUCKETS];

            build_infos.iter().for_each(|build_info| {
                let idx = bucket_index(build_info, dim);
                buckets[usize::of(idx)].cost += build_info.isect_cost;
                buckets[usize::of(idx)].bbox
                    = buckets[usize::of(idx)].bbox | build_info.bbox;
            });

            let cost_of_split = |(a, b): (&[Bucket], &[Bucket])| {
                let range_cost = |r: &[Bucket]| r.iter().fold((0., BBox::ZERO),
                    |(c, bb), Bucket { cost, bbox }| (c + cost, bb | *bbox));

                let (cost1, bbox1) = range_cost(a);
                let (cost2, bbox2) = range_cost(b);

                1. + F2::dot(A2(cost1, cost2),
                             A2(&bbox1, &bbox2).map(BBox::surface_area))
                   / bbox.surface_area()
            };

            (1..NUM_BUCKETS - 1).map(|i| (i, buckets.split_at(i.conv())))
                                .map(|(idx, bb)| (cost_of_split(bb), idx))
                                .fold((F::POS_INF, 0), |(a, b), (c, d)| {
                                    if a < c { (a, b) }
                                    else { (c, d) }
                                })
        }).zip(XYZ, |a, b| (a, b))
          .fold((F::POS_INF, 0, X), |(cost, idx, dim), ((c, i), d)|
                if c < cost { (c, i, d) } else { (cost, idx, dim) });

        (mc_dim, partition(build_infos,
                           |build_info| bucket_index(build_info, mc_dim)
                                        < mc_idx.conv()))
    };

    let tree_l = build(&mut build_infos[..usize::of(pivot)], idx_map, arena);
    let tree_r = build(&mut build_infos[usize::of(pivot)..], idx_map, arena);

    arena.alloc(BuildNode {
        bbox,
        sizel: tree_l.size(),
        sizer: tree_r.size(),
        node: BuildNodeType::Tree(dim, tree_l, tree_r),
    })
}

impl<S> Intersectable for BVH<S> where S: Intersectable
{
    #[inline] fn bbox(&self) -> BBox { self.nodes[0].bbox }

    #[inline] fn intersects(&self, ray: R) -> bool {
        self.fold(conv!(ray.d => F3).map(F::is_sign_positive),
                  false,
                  |_, node| node.bbox.intersects(ray),
                  |_, _, isectable| {
                      if isectable.intersects(ray) {
                          Either::L(true)
                      } else {
                          Either::R(false)
                      }
                  })
    }

    #[inline] fn intersect(&self, ray: R) -> Option<Its> {
        self.fold(conv!(ray.d => F3).map(F::is_sign_positive),
                  (ray, None),
                  |(r, _), node| node.bbox.intersects(*r),
                  |acc, _, s| Either::R(intersect_update(acc, s)))
            .1
            .map(Its::with_hit_info)
    }

    fn hit_info(&self, _: Its) -> Its { unreachable!() }
    fn sample_surface(&self, _: F2) -> Its { unreachable!() }
    fn surface_area(&self) -> F { unreachable!() }

    fn intersection_cost(&self) -> F {
        2. * F::of(self.nodes.len()).log2()
            .mul_add(BBox::ZERO.intersection_cost(),
                     self.elements[0].intersection_cost())
    }
}

#[inline] pub fn intersect_update((ray, acc): (R, Option<Its>),
                                  s: &impl Intersectable) -> (R, Option<Its>) {
    s.intersect(ray)
     .map_or_else(|| (ray, acc), |it| (ray.clipped(it.t), Some(it)))
}

fn partition<E>(items: &mut [E], pred: impl Fn(&E) -> bool) -> I {
    let mut pivot = 0;
    let mut it = items.iter_mut();
    'main: while let Some(i) = it.next() {
        if !pred(i) {
            loop {
                match it.next_back() {
                    Some(j) => { if pred(j) { mem::swap(i, j); break } }
                    None => break 'main,
                }
            }
        }
        pivot += 1;
    }
    pivot
}
