use super::*;


pub enum BVH {}


/*
pub type Sequence = Aggregate<Structure, Vec<Structure>>;

pub struct Aggregate<E, C> where for<'a> &'a C: IntoIterator<Item=&'a E> {
    pub elements: C,
}

impl<E, C> Aggregate<E, C> where for<'a> &'a C: IntoIterator<Item=&'a E> {
    #[inline]
    pub fn new(elements: C) -> Aggregate<E, C> {
        Aggregate { elements }
    }

}

impl<E, C> Intersectable for Aggregate<E, C>
        where E: Intersectable,
              for<'a> &'a C: IntoIterator<Item=&'a E> {
    #[inline]
    fn bbox(&self, t: T) -> BBox {
        self.elements.into_iter().fold(BBox::EMPTY, |bbox, e| bbox | e.bbox(t))
    }

    #[inline]
    fn intersects(&self, ray: R) -> bool {
        self.elements.into_iter().any(|e| e.intersects(ray))
    }

    #[inline]
    fn intersect(&self, ray: R) -> Option<Its> {
        self.elements.into_iter().fold(None, |its, e| {
            let r = match &its {
                None => ray,
                Some(it) => ray.clip_max(it.t),
            };
            e.intersect(r).or(its)
        })
    }
}
*/
