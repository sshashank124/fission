use std::collections::HashMap;
use std::mem;

use staticvec::StaticVec;

use super::*;

pub struct BVH<S> {
    nodes:        Vec<BVHNode>,
    pub elements: Vec<S>,
}

pub struct BVHNode {
    pub bbox: BBox,
    node:     BVHNodeType,
}

pub enum BVHNodeType {
    Leaf(I, I),
    Tree(I, Dim),
}

impl<S> BVH<S> where S: Intersectable
{
    pub fn new(elems: Vec<S>) -> Self {
        assert!(!elems.is_empty());

        let mut build_infos = elems.iter()
                                   .enumerate()
                                   .map(|(idx, e)| {
                                       let bbox = e.bbox();
                                       BuildInfo { bbox,
                                                   center: bbox.center(),
                                                   idx: idx as I,
                                                   isect_cost:
                                                       e.intersection_cost() }
                                   })
                                   .collect::<Vec<_>>();

        let mut idx_map = HashMap::with_capacity(elems.len());
        let root = build(&mut build_infos[..], &mut idx_map);

        let mut nodes = Vec::with_capacity(root.size() as usize);
        flatten_tree(&root, &mut nodes, 0);

        let idx_ord = (0..elems.len() as I).map(|i| idx_map[&i]);
        let mut elems_idx = elems.into_iter().zip(idx_ord).collect::<Vec<_>>();
        elems_idx.sort_unstable_by_key(|(_, i)| *i);
        let elements =
            elems_idx.into_iter().map(|(e, _)| e).collect::<Vec<_>>();

        Self { elements, nodes }
    }

    #[inline(always)]
    pub fn fold<'a, A>(&'a self,
                       trav_order: A3<bool>,
                       mut acc: A,
                       pred: impl Fn(&mut A, &BVHNode) -> bool,
                       f: impl Fn(A, usize, &'a S) -> Either<A, A>)
                       -> A {
        let mut idx = 0;
        let mut stack = StaticVec::<I, 32>::new();
        loop {
            let node = &self.nodes[idx as usize];
            if pred(&mut acc, node) {
                match node.node {
                    BVHNodeType::Tree(ri, dim) => {
                        let ii = A2(ri, idx + 1);
                        stack.push(ii[!trav_order[dim]]);
                        idx = ii[trav_order[dim]];
                    }
                    BVHNodeType::Leaf(i, n) => {
                        for j in i as usize..(i + n) as usize {
                            acc = match f(acc, j, &self.elements[j]) {
                                Either::L(b) => return b,
                                Either::R(a) => a,
                            };
                        }
                        idx = match stack.pop() {
                            Some(i) => i,
                            _ => break
                        };
                    }
                }
            } else {
                idx = match stack.pop() {
                    Some(i) => i,
                    _ => break
                };
            }
        }
        acc
    }
}

struct BuildNode {
    bbox:  BBox,
    node:  BuildNodeType,
    sizel: I,
    sizer: I,
}

enum BuildNodeType {
    Leaf(I, I),
    Tree(Dim, Box<BuildNode>, Box<BuildNode>),
}

struct BuildInfo {
    bbox:       BBox,
    center:     P,
    idx:        I,
    isect_cost: F,
}

impl BuildNode {
    fn size(&self) -> I { self.sizel + self.sizer + 1 }
}

fn flatten_tree(tree: &BuildNode, nodes: &mut Vec<BVHNode>, mut offset: I) {
    offset += 1;
    let node = match tree.node {
        BuildNodeType::Leaf(idx, n) => BVHNodeType::Leaf(idx, n),
        BuildNodeType::Tree(dim, ref treel, _) => {
            BVHNodeType::Tree(offset + treel.size(), dim)
        }
    };

    nodes.push(BVHNode { bbox: tree.bbox, node });

    if let BuildNodeType::Tree(_, treel, treer) = &tree.node {
        flatten_tree(treel, nodes, offset);
        flatten_tree(treer, nodes, offset + treel.size());
    }
}

const NUM_BUCKETS: I = 24;

#[derive(Clone, Copy)]
struct Bucket {
    cost: F,
    bbox: BBox,
}

const MAX_LEAF_LEN: I = 4;

fn build(build_infos: &mut [BuildInfo],
         idx_map: &mut HashMap<I, I>)
         -> BuildNode {
    let n = build_infos.len() as I;

    if n <= MAX_LEAF_LEN {
        build_infos.iter().for_each(|bi| {
                              idx_map.insert(bi.idx, idx_map.len() as I);
                          });
        return BuildNode { bbox:
                               build_infos.iter().fold(BBox::ZERO, |bb, bi| {
                                                     bb | bi.bbox
                                                 }),
                           node:  BuildNodeType::Leaf(idx_map.len() as I - n,
                                                      n),
                           sizel: 0,
                           sizer: 0, }
    }

    let (bbox, centers_bbox) =
        build_infos.iter().fold((BBox::ZERO, BBox::ZERO), |(bb, bc), b| {
                              (bb | b.bbox, bc | b.center)
                          });

    let (extent, dim) = centers_bbox.max_extent();

    let pivot = if F::approx_zero(extent) {
        n / 2
    } else {
        let lb = centers_bbox[dim][0];
        let mut buckets =
            [Bucket { cost: 0., bbox: BBox::ZERO }; NUM_BUCKETS as usize];

        let bucket_index = |build_info: &BuildInfo| {
            let idx = (NUM_BUCKETS as F
                       * ((build_info.center[dim] - lb) / extent))
                      as I;
            idx.min(NUM_BUCKETS - 1)
        };

        build_infos.iter().for_each(|build_info| {
                              let idx = bucket_index(build_info);
                              buckets[idx as usize].cost +=
                                  build_info.isect_cost;
                              buckets[idx as usize].bbox =
                                  buckets[idx as usize].bbox | build_info.bbox;
                          });

        let cost_of_split = |(a, b): (&[Bucket], &[Bucket])| {
            let range_cost = |r: &[Bucket]| {
                r.iter()
                 .fold((0., BBox::ZERO), |(c, bb), Bucket { cost, bbox }| {
                     (c + cost, bb | *bbox)
                 })
            };

            let (cost1, bbox1) = range_cost(a);
            let (cost2, bbox2) = range_cost(b);

            1.
            + (cost1 * bbox1.surface_area() + cost2 * bbox2.surface_area())
              / bbox.surface_area()
        };


        let mc_idx =
            (1..NUM_BUCKETS - 1).map(|i| (i, buckets.split_at(i as usize)))
                                .map(|(idx, bb)| (cost_of_split(bb), idx))
                                .fold((F::POS_INF, 0), |(a, b), (c, d)| {
                                    if a < c {
                                        (a, b)
                                    } else {
                                        (c, d)
                                    }
                                })
                                .1;

        partition(build_infos, |build_info| bucket_index(build_info) < mc_idx)
    };

    let treel = build(&mut build_infos[..pivot as usize], idx_map);
    let treer = build(&mut build_infos[pivot as usize..], idx_map);

    BuildNode { bbox,
                sizel: treel.size(),
                sizer: treer.size(),
                node: BuildNodeType::Tree(dim,
                                          Box::new(treel),
                                          Box::new(treer)) }
}

impl<S> Intersectable for BVH<S> where S: Intersectable
{
    fn bbox(&self) -> BBox {
        self.elements.iter().fold(BBox::ZERO, |bbox, e| bbox | e.bbox())
    }

    #[inline(always)]
    fn intersects(&self, ray: R) -> bool {
        self.fold(ray.d.map(Num::is_pos),
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

    #[inline(always)]
    fn intersect(&self, ray: R) -> Option<Its> {
        self.fold(ray.d.map(Num::is_pos),
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
        2.
        * ((self.nodes.len() as F).log2() * BBox::ZERO.intersection_cost()
           + self.elements[0].intersection_cost())
    }
}

type Acc<'a> = (R, Option<Its<'a>>);
#[inline(always)]
pub fn intersect_update<'a>((ray, acc): Acc<'a>,
                            s: &'a impl Intersectable)
                            -> Acc<'a> {
    s.intersect(ray)
     .map(|it| (ray.clipped(it.t), Some(it)))
     .unwrap_or_else(|| (ray, acc))
}

pub fn partition<E>(items: &mut [E], pred: impl Fn(&E) -> bool) -> I {
    let mut pivot = 0;
    let mut it = items.iter_mut();
    'main: while let Some(i) = it.next() {
        if !pred(&i) {
            loop {
                match it.next_back() {
                    Some(j) => {
                        if pred(j) {
                            mem::swap(i, j);
                            break
                        }
                    }
                    None => break 'main,
                }
            }
        }
        pivot += 1;
    }
    pivot
}
