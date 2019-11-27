mod triangle;

use either::Either;

use super::*;

pub use triangle::*;


pub struct Mesh(BVH<Triangle>);

impl Mesh {
    #[inline(always)] pub fn new(triangles: Vec<Triangle>) -> Self
    { Self(BVH::new(triangles)) }
}

impl Intersectable for Mesh {
    #[inline(always)] fn bbox(&self) -> BBox { self.0.bbox() }

    #[inline(always)] fn intersects(&self, ray: R) -> bool
    { self.0.intersects(ray) }

    #[inline(always)] fn intersect(&self, ray: R) -> Option<Its> {
        self.0.fold(ray.d.map(Num::is_pos), (ray, None),
                    |(r, _), node| node.bbox.intersects(*r),
                    |acc, i, s| Either::Right(intersect_update(acc, (i, s)))).1
    }

    #[inline(always)] fn hit_info<'a>(&'a self, i: Its<'a>) -> Its<'a>
    { self.0.elements[i.shape.1 as usize].hit_info(i) }

    #[inline(always)] fn intersection_cost(&self) -> F
    { self.0.intersection_cost() }
}

type Acc<'a> = (R, Option<Its<'a>>);
#[inline(always)] pub fn intersect_update<'a, S>((ray, acc): Acc<'a>,
                                                 (i, s): (usize, &'a S))
    -> Acc<'a> where S: Intersectable
{ s.intersect(ray).map(|it| (ray.clipped(it.t), Some(it.for_idx(i))))
                  .unwrap_or_else(|| (ray, acc)) }
