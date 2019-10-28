use std::ops::{BitOr, Mul, Div};

use super::*;


#[derive(Clone, Copy)]
pub struct BBox(pub A3<B>);

impl BBox {
    #[inline]
    pub fn cube(b: B) -> BBox {
        BBox(rep(b))
    }

    #[inline]
    pub fn intersects(&self, R{o, d, tb}: R) -> bool {
        (self.0).0.intersect(o.x(), d.x(), tb)
            .and_then(|b| { (self.0).1.intersect(o.y(), d.y(), b) })
            .and_then(|b| { (self.0).2.intersect(o.z(), d.z(), b) })
            .is_some()
    }

    pub const EMPTY: BBox = BBox(A3(B::EMPTY, B::EMPTY, B::EMPTY));
}

impl BitOr for BBox {
    type Output = BBox;
    #[inline]
    fn bitor(self, BBox(bbox): BBox) -> BBox {
        BBox(zip(self.0, bbox, BitOr::bitor))
    }
}

impl BitOr<P> for BBox {
    type Output = BBox;
    #[inline]
    fn bitor(self, P(p): P) -> BBox {
        BBox(zip(self.0, p, BitOr::bitor))
    }
}

impl Mul<BBox> for T {
    type Output = BBox;
    #[inline]
    fn mul(self, bbox: BBox) -> BBox {
        BBox(self * bbox.0)
    }
}

impl Div<BBox> for T {
    type Output = BBox;
    #[inline]
    fn div(self, bbox: BBox) -> BBox {
        BBox(self / bbox.0)
    }
}
