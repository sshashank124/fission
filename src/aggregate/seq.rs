use std::mem;

use super::*;


impl<S> Intersectable for &[S] where S: Intersectable {
    #[inline(always)] fn bbox(&self) -> BBox
    { self.iter().fold(BBox::ZERO, |bbox, e| bbox | e.bbox()) }

    #[inline(always)] fn intersects(&self, ray: R) -> bool
    { self.iter().any(|s| s.intersects(ray)) }

    #[inline(always)] fn intersect(&self, ray: R) -> Option<Its>
    { self.iter().enumerate().fold((ray, None), intersect_update)
                 .1.map(Its::with_hit_info) }
    
    #[inline(always)] fn hit_info<'a>(&'a self, i: Its<'a>) -> Its<'a>
    { self[i.shape.1 as usize].hit_info(i) }

    #[inline(always)] fn intersection_cost(&self) -> F
    { self.len() as F * self[0].intersection_cost() }
}

type Acc<'a> = (R, Option<Its<'a>>);
#[inline(always)] pub fn intersect_update<'a, S>((ray, acc): Acc<'a>,
                                                 (i, s): (usize, &'a S))
    -> Acc<'a> where S: Intersectable
{ s.intersect(ray).map(|it| (ray.clipped(it.t), Some(it.for_idx(i as I))))
                  .unwrap_or_else(|| (ray, acc)) }

#[inline(always)] pub fn partition<E, FN>(items: &mut [E], pred: FN) -> I
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
