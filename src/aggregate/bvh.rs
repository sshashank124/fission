use std::mem;

use either::Either;

use super::*;


#[derive(Debug)]
pub struct BVH<S> {
    elements: Vec<S>,
    nodes: Vec<BVHNode>,
}

#[derive(Debug)]
struct BVHNode {
    bbox: BBox,
    node: BVHNodeType,
}

#[derive(Debug)]
enum BVHNodeType {
    Leaf(usize),
    Tree(Axis, usize),
}
use BVHNodeType::*;

#[derive(Debug)]
struct BuildNode {
    bbox: BBox,
    node: BuildNodeType,
    sizel: usize,
    sizer: usize,
}

#[derive(Debug)]
enum BuildNodeType {
    Leaf(usize),
    Tree(Axis, Box<BuildNode>, Box<BuildNode>),
}

struct BuildInfo {
    bbox: BBox,
    center: P,
    idx: usize,
}

macro_rules! pop_break {
    ($stack:ident) => {
        match $stack.pop() {
            None => break,
            Some(v) => v,
        }
    }
}

impl<S> BVH<S> where S: Intersectable {
    pub fn new(elements: Vec<S>) -> BVH<S> {
        assert!(!elements.is_empty());
        let mut build_infos = elements.iter().enumerate().map(|(idx, e)| {
            let bbox = e.bbox(T::ONE);
            BuildInfo { bbox, center: bbox.center(), idx }
        }).collect::<Vec<_>>();

        let root = build(&mut build_infos[..]);
        let mut nodes = Vec::with_capacity(root.size());

        flatten_tree(&root, &mut nodes, 0);

        BVH { elements, nodes }
    }

    #[inline(always)]
    fn fold<A, P, F>(&self, trav_order: A3<bool>,
                  acc: A, pred: P, f: F) -> A
            where P: Fn(&A, &BVHNode) -> bool,
                  F: Fn(&A, &S) -> Either<A, A> {
        let mut acc = acc;
        let mut idx = 0;
        let mut stack = Vec::with_capacity(64);
        loop {
            let node = &self.nodes[idx];
            if pred(&acc, node) {
                match &node.node {
                    BVHNodeType::Leaf(i) => {
                        match f(&acc, &self.elements[*i]) {
                            Either::Left(b) => { return b; },
                            Either::Right(a) => { acc = a; },
                        }
                        idx = pop_break!(stack);
                    },
                    BVHNodeType::Tree(axis, ri) => {
                        idx = if trav_order[*axis] {
                            stack.push(*ri); idx + 1
                        } else {
                            stack.push(idx + 1); *ri
                        };
                    },
                }
            } else {
                idx = pop_break!(stack);
            }
        }
        acc
    }
}

impl BuildNode {
    #[inline(always)]
    fn size(&self) -> usize { self.sizel + self.sizer + 1 }
}

fn flatten_tree(tree: &BuildNode, nodes: &mut Vec<BVHNode>, offset: usize) {
    let offset = offset + 1;
    let node = match &tree.node {
        BuildNodeType::Leaf(idx) => Leaf(*idx),
        BuildNodeType::Tree(axis, treel, _) => {
            Tree(*axis, offset + treel.size())
        }
    };

    nodes.push(BVHNode { bbox: tree.bbox, node });

    if let BuildNodeType::Tree(_, treel, treer) = &tree.node {
        flatten_tree(treel, nodes, offset);
        flatten_tree(treer, nodes, offset + treel.size());
    }
}

const NUM_BUCKETS: usize = 16;

#[derive(Clone, Copy)]
struct Bucket {
    n: I,
    bbox: BBox,
}

fn build(build_infos: &mut [BuildInfo]) -> BuildNode {
    let n = build_infos.len();

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

    let (axis, extent) = centers_bbox.max_extent();

    let pivot = if extent < F::EPSILON { n / 2 }
    else {
        let B(lb, _) = centers_bbox[axis];
        let mut buckets = [Bucket { n: 0, bbox: BBox::ZERO }; NUM_BUCKETS];

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
            let (n1, bbox1) = a.iter().fold((0, BBox::ZERO),
                                  |(c, bb), Bucket { n, bbox }|
                                      (c + n, bb | *bbox));

            let (n2, bbox2) = b.iter().fold((0, BBox::ZERO),
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

    let treel = build(&mut build_infos[..pivot]);
    let treer = build(&mut build_infos[pivot..]);

    BuildNode {
        bbox,
        sizel: treel.size(), sizer: treer.size(),
        node: BuildNodeType::Tree(axis, Box::new(treel), Box::new(treer)),
    }
}

fn partition<E, FN>(items: &mut [E], pred: FN) -> usize
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
    #[inline(always)]
    fn bbox(&self, t: T) -> BBox {
        t * self.nodes[0].bbox
    }

    #[inline(always)]
    fn intersects(&self, ray: R) -> bool {
        self.fold(ray.d.map(|i| i > 0.), false,
                  |_, node| node.bbox.intersects(ray),
                  |_, isectable| {
                      if isectable.intersects(ray) { Either::Left(true) }
                      else { Either::Right(false) }
                  })
    }

    #[inline(always)]
    fn intersect(&self, ray: R) -> Option<Its> {
        self.fold(ray.d.map(|i| i > 0.), (ray, None),
                  |(ray, _), node| node.bbox.intersects(*ray),
                  |(ray, its), isectable| {
                      let it = isectable.intersect(*ray);
                      let ray = ray.clip_from_its(&it);
                      Either::Right((ray, it.or(*its)))
                  }).1
    }
}
