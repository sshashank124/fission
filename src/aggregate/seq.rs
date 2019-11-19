use std::mem;

use super::*;


impl<S> Intersectable for &[S] where S: Intersectable {
    #[inline(always)] fn bbox(&self, t: T) -> BBox
    { self.iter().fold(BBox::ZERO, |bbox, e| bbox | e.bbox(t)) }

    #[inline(always)] fn intersects(&self, ray: R) -> bool
    { self.iter().any(|s| s.intersects(ray)) }

    #[inline(always)]
    fn intersect(&self, ray: R) -> Option<Its> {
        let (_, acc, i) = self.iter().fold((ray, None, 0), intersect_update);
        acc.map(|(closest, mut its)| { its.i = i; closest.hit_info(its) })
    }
    
    #[inline(always)] fn hit_info(&self, its: Its) -> Its { its }

    #[inline(always)] fn intersection_cost(&self) -> F
    { self.len() as F * self[0].intersection_cost() }
}

#[inline(always)]
pub fn intersect_update<'a, S>((ray, acc, i): (R, Option<(&'a S, Its)>, I),
                               s: &'a S)
    -> (R, Option<(&'a S, Its)>, I) where S: Intersectable {
    s.intersect(ray).map(|it| {
        let i = i + it.i + 1;
        (ray.clipped(it.t), Some((s, it)), i)
    }).unwrap_or_else(|| (ray, acc, i + 1))
}

#[inline(always)]
pub fn partition<E, FN>(items: &mut [E], pred: FN) -> I
        where FN: Fn(&E) -> bool {
    let mut pivot = 0;
    let mut it = items.iter_mut();
    'main: while let Some(i) = it.next() {
        if !pred(&i) { loop {
            match it.next_back() {
                Some(j) => if pred(j) { mem::swap(i, j); break; },
                None => break 'main,
            }
        } }
        pivot += 1;
    }
    pivot
}
