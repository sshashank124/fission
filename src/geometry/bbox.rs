use std::ops::{BitOr, Mul, Div, Deref};

use super::*;
use crate::op;


#[derive(Clone, Copy)]
pub struct BBox(pub A3<B>);

impl Zero for BBox {
    const ZERO: Self = BBox(A3(B::ZERO, B::ZERO, B::ZERO));
}

impl BBox {
    #[inline(always)]
    pub fn center(&self) -> P {
        P(self.map(B::center))
    }

    #[inline(always)]
    pub fn extents(&self) -> F3 {
        self.map(B::extent)
    }

    #[inline(always)]
    pub fn max_extent(&self) -> (Axis, F) {
        let A3(xe, ye, ze) = self.extents();
        if ye > xe {
            if ze > ye { (Z, ze) }
            else { (Y, ye) }
        } else if ze > xe {
            (Z, ze)
        } else {
            (X, xe)
        }
    }

    #[inline(always)]
    pub fn surface_area(&self) -> F {
        let A3(xe, ye, ze) = self.extents();
        2. * xe * ye + 2. * xe * ze + 2. * ye * ze
    }
}

impl Intersectable for BBox {
    #[inline(always)] fn bbox(&self, _: T) -> BBox { *self }

    #[inline(always)]
    fn intersects(&self, ray: &R) -> bool {
        let mut b = ray.tb & ((self[X] - ray.o[X]) * ray.d_inv[X]);
        if b.degen() { return false; }
        b = b & ((self[Y] - ray.o[Y]) * ray.d_inv[Y]);
        if b.degen() { return false; }
        b = b & ((self[Z] - ray.o[Z]) * ray.d_inv[Z]);
        !b.degen()
    }

    #[inline(always)] fn intersect(&self, _: &mut R) -> Option<Its> { None }

    #[inline(always)] fn hit_info(&self, _: &mut Its) { }
}

impl BitOr for BBox {
    type Output = BBox;
    #[inline(always)]
    fn bitor(self, bbox: BBox) -> BBox {
        BBox(zip(*self, *bbox, BitOr::bitor))
    }
}

impl BitOr<P> for BBox {
    type Output = BBox;
    #[inline(always)]
    fn bitor(self, p: P) -> BBox {
        BBox(zip(*self, *p, BitOr::bitor))
    }
}

op!(Mul::mul, T -> *BBox -> BBox);
op!(Div::div, T -> *BBox -> BBox);

impl Deref for BBox {
    type Target = A3<B>;
    #[inline(always)] fn deref(&self) -> &Self::Target { &self.0 }
}
