use super::*;


impl<S> Intersectable for Vec<S> where S: Intersectable {
    #[inline]
    fn bbox(&self, t: T) -> BBox {
        self.iter().fold(BBox::EMPTY, |bbox, e| bbox | e.bbox(t))
    }

    #[inline]
    fn intersects(&self, ray: R) -> bool {
        self.iter().any(|e| e.intersects(ray))
    }

    #[inline]
    fn intersect(&self, ray: R) -> Option<Its> {
        self.iter().fold(None, |its, e| {
            let r = match &its {
                None => ray,
                Some(it) => ray.clip_max(it.t),
            };
            e.intersect(r).or(its)
        })
    }
}
