use std::mem;

use super::*;


#[derive(Debug)]
pub struct BVH<S> {
    elements: Vec<S>,
    node: BVHNode,
}

#[derive(Debug)]
struct BVHNode {
    bbox: BBox,
    node: BVHNodeType,
}

#[derive(Debug)]
enum BVHNodeType {
    Leaf(usize),
    Tree(Axis, Box<BVHNode>, Box<BVHNode>),
}
use BVHNodeType::*;

struct BuildInfo {
    bbox: BBox,
    center: P,
    idx: usize,
}

impl<S> BVH<S> where S: Intersectable {
    pub fn new(elements: Vec<S>) -> BVH<S> {
        assert!(!elements.is_empty());
        let mut build_infos = elements.iter().enumerate().map(|(idx, e)| {
            let bbox = e.bbox(T::I);
            BuildInfo { bbox, center: bbox.center(), idx }
        }).collect::<Vec<_>>();

        BVH {
            elements,
            node: build(&mut build_infos[..]),
        }
    }

    #[inline(always)]
    fn intersects(&self, ray: R, node: &BVHNode) -> bool {
        if !node.bbox.intersects(ray) { return false; }

        match &node.node {
            Leaf(idx) => self.elements[*idx].intersects(ray),
            Tree(axis, left, right) => {
                let (na, nb) = if ray.d[*axis] > 0. { (left, right) }
                               else { (right, left) };
                self.intersects(ray, na) || self.intersects(ray, nb)
            }
        }
    }

    #[inline(always)]
    fn intersect(&self, ray: R, node: &BVHNode) -> Option<Its> {
        if !node.bbox.intersects(ray) { return None; }

        match &node.node {
            Leaf(idx) => self.elements[*idx].intersect(ray),
            Tree(axis, left, right) => {
                let (na, nb) = if ray.d[*axis] > 0. { (left, right) }
                               else { (right, left) };
                let its = self.intersect(ray, na);
                self.intersect(ray.clip_from_its(&its), nb).or(its)
            }
        }
    }
}

const NUM_BUCKETS: usize = 16;

#[derive(Clone, Copy)]
struct Bucket {
    n: I,
    bbox: BBox,
}

fn build(build_infos: &mut [BuildInfo]) -> BVHNode {
    let n = build_infos.len();

    if n == 1 {
        return BVHNode {
            bbox: build_infos[0].bbox,
            node: Leaf(build_infos[0].idx),
        };
    }

    let (bbox, centers_bbox) =
        build_infos.iter().fold((BBox::EMPTY, BBox::EMPTY), |(bb, bc), b| {
            (bb | b.bbox, bc | b.center)
        });

    let (axis, extent) = centers_bbox.max_extent();

    let pivot = if extent < F::EPSILON { n / 2 }
    else {
        let B(lb, _) = centers_bbox[axis];
        let mut buckets = [Bucket { n: 0, bbox: BBox::EMPTY }; NUM_BUCKETS];

        let bucket_index = |build_info: &BuildInfo| {
            let idx = (NUM_BUCKETS as F *
                       ((build_info.center[axis] - lb) / extent)) as usize;
            idx.min(NUM_BUCKETS - 1)
        };

        build_infos.iter().for_each(|build_info| {
            let idx = bucket_index(build_info);
            buckets[idx].n += 1;
            buckets[idx].bbox = buckets[idx].bbox | build_info.bbox;
        });

        let cost_of_split = |(a, b): (&[Bucket], &[Bucket])| {
            let (n1, bbox1) = a.iter().fold((0, BBox::EMPTY),
                                  |(c, bb), Bucket { n, bbox }|
                                      (c + n, bb | *bbox));

            let (n2, bbox2) = b.iter().fold((0, BBox::EMPTY),
                                  |(c, bb), Bucket { n, bbox }|
                                      (c + n, bb | *bbox));

            1. + (n1 as F * bbox1.surface_area() +
                  n2 as F * bbox2.surface_area()) /
                 bbox.surface_area()
        };

        let min_cost_idx = (1..NUM_BUCKETS-1)
                               .map(|idx| (idx, buckets.split_at(idx)))
                               .map(|(idx, bb)| (idx, cost_of_split(bb)))
                               .fold((0, F::POS_INF), |(ii, cc), (i, c)| {
                                   if c < cc { (i, c) } else { (ii, cc) }
                               }).0;

        partition(build_infos,
                  |ref build_info| bucket_index(build_info) < min_cost_idx)
    };

    BVHNode {
        bbox,
        node: Tree(axis,
                   Box::new(build(&mut build_infos[..pivot])),
                   Box::new(build(&mut build_infos[pivot..]))),
    }
}

fn partition<E, FN>(items: &mut [E], pred: FN) -> usize
        where FN: Fn(&E) -> bool {
    let mut pivot = 0;
    let mut it = items.iter_mut();
    'main: while let Some(i) = it.next() {
        if !pred(i) {
            loop {
                match it.next_back() {
                    Some(j) => if pred(j) {
                        mem::swap(i, j);
                        break;
                    },
                    None => break 'main,
                }
            }
        }
        pivot += 1;
    }
    pivot
}

impl<S> Intersectable for BVH<S> where S: Intersectable {
    #[inline(always)]
    fn bbox(&self, t: T) -> BBox {
        t * self.node.bbox
    }

    #[inline(always)]
    fn intersects(&self, ray: R) -> bool {
        self.intersects(ray, &self.node)
    }

    #[inline(always)]
    fn intersect(&self, ray: R) -> Option<Its> {
        self.intersect(ray, &self.node)
    }
}
