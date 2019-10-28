use super::*;

use crate::solver;


pub struct Sphere {
    pub r: F,
}

impl Sphere {
    #[inline]
    pub fn new(r: F) -> Sphere {
        Sphere {
            r,
        }
    }

    #[inline]
    fn intersection_point(&self, R{o, d, tb}: R) -> Option<F> {
        solver::quad(d.norm2(),
                     2. * d.dot(V::p(o)),
                     V::p(o).norm2() - self.r * self.r)
            .and_then(|P2(t1, t2)| {
                if tb.bounds(t1) {
                    Some(t1)
                } else if tb.bounds(t2) {
                    Some(t2)
                } else {
                    None
                }
            })
    }
}

impl Intersectable for Sphere {
    #[inline]
    fn bbox(&self, t: T) -> BBox {
        t * BBox::cube(B(-self.r, self.r))
    }

    #[inline]
    fn intersects(&self, r: R) -> bool {
        self.intersection_point(r).is_some()
    }

    #[inline]
    fn intersect(&self, r: R) -> Option<Its> {
        self.intersection_point(r).map(|t| {
            let p = r.at(t);
            Its::ideal(p, t, N(V::p(p)))
        })
    }
}
