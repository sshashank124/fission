use std::mem;

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

impl<S> BVH<S> where S: Intersectable {
    pub fn new(elements: Vec<S>) -> BVH<S> {
        assert!(!elements.is_empty());
        let mut build_infos = elements.iter().enumerate().map(|(idx, e)| {
            let bbox = e.bbox(T::I);
            BuildInfo { bbox, center: bbox.center(), idx }
        }).collect::<Vec<_>>();

        let root = build(&mut build_infos[..]);
        let num_nodes = root.sizel + root.sizer + 1;
        let mut nodes = Vec::with_capacity(num_nodes);

        flatten_tree(&root, &mut nodes, 0);

        BVH { elements, nodes }
    }
}

fn flatten_tree(tree: &BuildNode, nodes: &mut Vec<BVHNode>, offset: usize) {
    let node = match &tree.node {
        BuildNodeType::Leaf(idx) => Leaf(*idx),
        BuildNodeType::Tree(axis, treel, _) => {
            Tree(*axis, offset + treel.sizel + treel.sizer + 2)
        }
    };

    nodes.push(BVHNode { bbox: tree.bbox, node });

    if let BuildNodeType::Tree(_, treel, treer) = &tree.node {
        flatten_tree(treel, nodes, offset + 1);
        flatten_tree(treer, nodes, offset + treel.sizel + treel.sizer + 2);
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
            sizel: 0,
            sizer: 0,
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

    let treel = build(&mut build_infos[..pivot]);
    let treer = build(&mut build_infos[pivot..]);

    BuildNode {
        bbox,
        sizel: treel.sizel + treel.sizer + 1,
        sizer: treer.sizel + treer.sizer + 1,
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
        let mut idx = 0;
        let mut stack = [0; 64];
        let mut sp = 0;
        loop {
            let node = &self.nodes[idx];
            if node.bbox.intersects(&ray) {
                match node.node {
                    BVHNodeType::Leaf(i) => {
                        if self.elements[i].intersects(ray) { return true; }
                        if sp == 0 { break; }
                        sp -= 1; idx = stack[sp];
                    },
                    BVHNodeType::Tree(axis, ridx) => {
                        if ray.d[axis] > 0. { stack[sp] = ridx; idx += 1; }
                        else { stack[sp] = idx + 1; idx = ridx; }
                        sp += 1;
                    },
                }
            } else {
                if sp == 0 { break; }
                sp -= 1; idx = stack[sp];
            }
        }
        false
    }

    #[inline(always)]
    fn intersect(&self, ray: R) -> Option<Its> {
        let mut ray = ray;
        let mut its = None;
        let mut idx = 0;
        let mut stack = [0; 64];
        let mut sp = 0;
        loop {
            let node = &self.nodes[idx];
            if node.bbox.intersects(&ray) {
                match &node.node {
                    BVHNodeType::Leaf(i) => {
                        let it = self.elements[*i].intersect(ray);
                        ray = ray.clip_from_its(&it);
                        its = it.or(its);
                        if sp == 0 { break; }
                        sp -= 1; idx = stack[sp];
                    },
                    BVHNodeType::Tree(axis, ridx) => {
                        if ray.d[*axis] > 0. { stack[sp] = *ridx; idx += 1; }
                        else { stack[sp] = idx + 1; idx = *ridx; }
                        sp += 1;
                    },
                }
            } else {
                if sp == 0 { break; }
                sp -= 1; idx = stack[sp];
            }
        }
        its
    }
}
