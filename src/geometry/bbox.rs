use std::ops::{BitOr, Index, Mul, Div};

use super::*;


#[derive(Clone, Copy, Debug)]
pub struct BBox(pub A3<B>);

impl BBox {
    #[inline(always)]
    pub fn cube(b: B) -> BBox {
        BBox(rep(b))
    }

    #[inline(always)]
    pub fn intersects(&self, ray: &R) -> bool {
        let mut b = ray.tb & ((self[Axis::X] - ray.o.x()) * ray.d_inv.x());
        if b.degen() { return false; }
        b = b & ((self[Axis::Y] - ray.o.y()) * ray.d_inv.y());
        if b.degen() { return false; }
        b = b & ((self[Axis::Z] - ray.o.z()) * ray.d_inv.z());
        !b.degen()
    }
    
    #[inline(always)]
    pub fn center(&self) -> P {
        P(self.0.map(B::center))
    }

    #[inline(always)]
    pub fn extents(&self) -> F3 {
        self.0.map(B::extent)
    }

    #[inline(always)]
    pub fn max_extent(&self) -> (Axis, F) {
        let A3(xe, ye, ze) = self.extents();
        if ye > xe {
            if ze > ye { (Axis::Z, ze) }
            else { (Axis::Y, ye) }
        } else if ze > xe {
            (Axis::Z, ze)
        } else {
            (Axis::X, xe)
        }
    }

    #[inline(always)]
    pub fn surface_area(&self) -> F {
        let A3(xe, ye, ze) = self.extents();
        2. * xe * ye + 2. * xe * ze + 2. * ye * ze
    }

    pub const EMPTY: BBox = BBox(A3(B::EMPTY, B::EMPTY, B::EMPTY));
}

impl BitOr for BBox {
    type Output = BBox;
    #[inline(always)]
    fn bitor(self, BBox(bbox): BBox) -> BBox {
        BBox(zip(self.0, bbox, BitOr::bitor))
    }
}

impl BitOr<P> for BBox {
    type Output = BBox;
    #[inline(always)]
    fn bitor(self, P(p): P) -> BBox {
        BBox(zip(self.0, p, BitOr::bitor))
    }
}

impl Mul<BBox> for T {
    type Output = BBox;
    #[inline(always)]
    fn mul(self, bbox: BBox) -> BBox {
        BBox(self * bbox.0)
    }
}

impl Div<BBox> for T {
    type Output = BBox;
    #[inline(always)]
    fn div(self, bbox: BBox) -> BBox {
        BBox(self / bbox.0)
    }
}

impl Index<Axis> for BBox {
    type Output = B;
    #[inline(always)]
    fn index(&self, axis: Axis) -> &B {
        &self.0[axis]
    }
}
