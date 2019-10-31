use super::*;


struct BVHNode {
    bbox: BBox,
    node: BVHNodeType,
}

enum BVHNodeType {
    Leaf(usize),      // index of element
    Interior(usize),  // index of right-subtree
}

pub struct BVH<S> {
    elements: Vec<S>,
    indices: Vec<BVHNode>,
}

impl<S> BVH<S> where S: Intersectable {
    pub fn new(elements: Vec<S>, t: T) {
        let bboxes: Vec<_> = elements.iter().map(|e| e.bbox(t)).collect();
        let centers: Vec<_> = bboxes.iter().map(|b| b.center()).collect();
        let mut indices: Vec<_> = (0..bboxes.len()).collect();
        build(&bboxes[..], &mut indices[..], 0);
    }
}

fn build(bboxes: &[BBox], indices: &mut [usize], offset: usize) {
    let (bbox, centers_bbox) =
        bboxes.iter().fold((BBox::EMPTY, BBox::EMPTY), |(bb, bc), b| {
            (bb | *b, bc | b.center())
        });
}

impl<S> Intersectable for BVH<S> where S: Intersectable {
    #[inline]
    fn bbox(&self, t: T) -> BBox {
        BBox::EMPTY
    }

    #[inline]
    fn intersects(&self, ray: R) -> bool {
        false
    }

    #[inline]
    fn intersect(&self, ray: R) -> Option<Its> {
        None
    }
}
