use super::*;


impl<S> Intersectable for Vec<S> where S: Intersectable {
    #[inline(always)]
    fn bbox(&self, t: T) -> BBox {
        self.iter().fold(BBox::EMPTY, |bbox, e| bbox | e.bbox(t))
    }

    #[inline(always)]
    fn intersects(&self, ray: R) -> bool {
        self.iter().any(|e| e.intersects(ray))
    }

    #[inline(always)]
    fn intersect(&self, ray: R) -> Option<Its> {
        self.iter().fold(None,
                         |its, e| e.intersect(ray.clip_from_its(&its)).or(its))
    }
}
