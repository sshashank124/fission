use std::mem;

use either::Either;

use crate::geometry::*;
use crate::shape::*;
use crate::util::*;


pub struct BVH<S> {
    nodes: Vec<BVHNode>,
    elements: Vec<S>,
}

pub struct BVHNode {
    bbox: BBox,
    node: BVHNodeType,
}

pub enum BVHNodeType {
    Leaf(I),
    Tree(Dim, I),
}

impl<S> BVH<S> where S: Intersectable {
    pub fn new(elements: Vec<S>) -> Self {
        assert!(!elements.is_empty());

        let mut build_infos = elements.iter().enumerate().map(|(idx, e)| {
            let bbox = e.bbox(T::ONE);
            BuildInfo {
                bbox,
                center: bbox.center(),
                idx: idx as I,
                isect_cost: e.intersection_cost(),
            }
        }).collect::<Vec<_>>();

        let root = build(&mut build_infos[..]);

        let mut nodes = Vec::with_capacity(root.size() as usize);
        flatten_tree(&root, &mut nodes, 0);

        Self { elements, nodes }
    }

    #[inline(always)]
    pub fn fold<'a, A, P, F>(&'a self, trav_order: A3<bool>,
                  mut acc: A, pred: P, f: F) -> A
            where P: Fn(&mut A, &BVHNode) -> bool,
                  F: Fn(A, &'a S) -> Either<A, A> {
        let mut idx = 0;
        let mut stack = [0; 32];
        let mut sp = 0;
        loop {
            let node = &self.nodes[idx as usize];
            if pred(&mut acc, node) {
                match &node.node {
                    BVHNodeType::Tree(split_dim, ri) => {
                        idx = if trav_order[*split_dim] {
                            stack[sp] = *ri; idx + 1
                        } else {
                            stack[sp] = idx + 1; *ri
                        };
                        sp += 1;
                    },
                    BVHNodeType::Leaf(i) => {
                        acc = match f(acc, &self.elements[*i as usize]) {
                            Either::Left(b) => { return b; },
                            Either::Right(a) => a,
                        };
                        idx = if sp == 0 { break }
                              else { sp -= 1; stack[sp] };
                    },
                }
            } else {
                idx = if sp == 0 { break }
                      else { sp -= 1; stack[sp] };
            }
        }
        acc
    }
}

struct BuildNode {
    bbox: BBox,
    node: BuildNodeType,
    sizel: I,
    sizer: I,
}

enum BuildNodeType {
    Leaf(I),
    Tree(Dim, Box<BuildNode>, Box<BuildNode>),
}

struct BuildInfo {
    bbox: BBox,
    center: P,
    idx: I,
    isect_cost: F,
}

impl BuildNode {
    #[inline(always)]
    fn size(&self) -> I { self.sizel + self.sizer + 1 }
}

fn flatten_tree(tree: &BuildNode, nodes: &mut Vec<BVHNode>, mut offset: I) {
    offset += 1;
    let node = match &tree.node {
        BuildNodeType::Leaf(idx) => BVHNodeType::Leaf(*idx),
        BuildNodeType::Tree(split_dim, treel, _) => {
            BVHNodeType::Tree(*split_dim, offset + treel.size())
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

fn build(build_infos: &mut [BuildInfo]) -> BuildNode {
    let n = build_infos.len() as I;

    if n == 1 {
        return BuildNode {
            bbox: build_infos[0].bbox,
            node: BuildNodeType::Leaf(build_infos[0].idx),
            sizel: 0, sizer: 0,
        };
    }

    let (bbox, centers_bbox) =
        build_infos.iter().fold((BBox::ZERO, BBox::ZERO), |(bb, bc), b| {
            (bb | b.bbox, bc | b.center)
        });

    let (extent, split_dim) = centers_bbox.max_extent();

    let pivot = if F::approx_zero(extent) { n / 2 }
    else {
        let lb = centers_bbox[split_dim][0];
        let mut buckets = [Bucket { cost: 0., bbox: BBox::ZERO };
                           NUM_BUCKETS as usize];

        let bucket_index = |build_info: &BuildInfo| {
            let idx = (NUM_BUCKETS as F *
                       ((build_info.center[split_dim] - lb) / extent)) as I;
            idx.min(NUM_BUCKETS - 1)
        };

        build_infos.iter().for_each(|build_info| {
            let idx = bucket_index(build_info);
            buckets[idx as usize].cost += build_info.isect_cost;
            buckets[idx as usize].bbox = buckets[idx as usize].bbox
                                       | build_info.bbox;
        });

        let cost_of_split = |(a, b): (&[Bucket], &[Bucket])| {
            let range_cost = |r: &[Bucket]| {
                r.iter().fold((0., BBox::ZERO),
                              |(c, bb), Bucket{ cost, bbox }|
                               (c + cost, bb | *bbox))
            };

            let (cost1, bbox1) = range_cost(a);
            let (cost2, bbox2) = range_cost(b);

            1. + (cost1 * bbox1.surface_area() +
                  cost2 * bbox2.surface_area()) /
                 bbox.surface_area()
        };

        let min_cost_idx = (1..NUM_BUCKETS-1)
                               .map(|i| (i, buckets.split_at(i as usize)))
                               .map(|(idx, bb)| (cost_of_split(bb), idx))
                               .fold((F::POS_INF, 0), tup_cmp_lt).1;

        partition(build_infos,
                  |ref build_info| bucket_index(build_info) < min_cost_idx)
    };

    let treel = build(&mut build_infos[..pivot as usize]);
    let treer = build(&mut build_infos[pivot as usize..]);

    BuildNode {
        bbox,
        sizel: treel.size(), sizer: treer.size(),
        node: BuildNodeType::Tree(split_dim, Box::new(treel), Box::new(treer)),
    }
}

fn partition<E, FN>(items: &mut [E], pred: FN) -> I
        where FN: Fn(&E) -> bool {
    let mut pivot = 0;
    let mut it = items.iter_mut();
    'main: while let Some(i) = it.next() {
        if !pred(i) { loop {
            match it.next_back() {
                Some(j) => if pred(j) { mem::swap(i, j); break; },
                None => break 'main,
            }
        } }
        pivot += 1;
    }
    pivot
}

impl<S> Intersectable for BVH<S> where S: Intersectable {
    #[inline(always)] fn bbox(&self, t: T) -> BBox
    { self.elements.iter().fold(BBox::ZERO, |bbox, e| bbox | e.bbox(t)) }

    #[inline(always)]
    fn intersects(&self, ray: R) -> bool {
        self.fold(ray.d.map(Num::is_pos), false,
                  |_, node| node.bbox.intersects(ray),
                  |_, isectable| {
                      if isectable.intersects(ray) { Either::Left(true) }
                      else { Either::Right(false) }
                  })
    }

    #[inline(always)]
    fn intersect(&self, ray: R) -> Option<Its> {
        let (_, acc, i) =
            self.fold(ray.d.map(Num::is_pos), (ray, None, 0),
                      |(ray, _, i), node| {
                          *i += 1;
                          node.bbox.intersects(*ray)
                      },
                      |(ray, acc, i), isectable| {
                          Either::Right(isectable.intersect(ray).map(|it| {
                              let i = i + it.i + 1;
                              (ray.clipped(it.t), Some((isectable, it)), i)
                          }).unwrap_or_else(|| (ray, acc, i + 1)))
                      });
        acc.map(|(closest, mut its)| { its.i = i; closest.hit_info(its) })

    }
    
    #[inline(always)] fn hit_info(&self, its: Its) -> Its { its }

    #[inline(always)] fn intersection_cost(&self) -> F
    { 2. * ((self.nodes.len() as F).log2() * BBox::ZERO.intersection_cost()
            + self.elements[0].intersection_cost()) }
}
