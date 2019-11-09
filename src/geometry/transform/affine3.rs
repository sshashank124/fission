use std::ops::{Add, Sub, Mul};

use super::*;
use rotscale3::RotScale3;


#[derive(Clone, Copy)]
pub struct Affine3 {
    r: RotScale3,
    t: F3,
}

impl Affine3 {
    #[inline(always)]
    pub const fn new(r: RotScale3, t: F3) -> Affine3 {
        Affine3 { r, t }
    }

    #[inline(always)]
    pub fn t(self) -> Affine3 {
        Affine3::new(self.r.t(), F3::ZERO)
    }

    #[inline(always)]
    pub fn rot(self) -> Affine3 {
        Affine3::new(self.r, F3::ZERO)
    }

    pub const I: Affine3 = Affine3::new(RotScale3::I, F3::ZERO);
}

impl Mul for Affine3 {
    type Output = Affine3;
    #[inline(always)]
    fn mul(self, m: Affine3) -> Affine3 {
        Affine3::new(self.r * m.r, self * m.t)
    }
}

impl<B, Z> Mul<A3<B>> for Affine3 where B: Copy + Mul<F, Output=Z>,
                                        Z: Add<Z, Output=Z>,
                                        Z: Add<F, Output=Z> {
    type Output = A3<Z>;
    #[inline(always)]
    fn mul(self, o: A3<B>) -> A3<Z> {
        zip(self.r * o, self.t, Add::add)
    }
}

impl Add<F3> for Affine3 {
    type Output = Affine3;
    #[inline(always)]
    fn add(self, v: F3) -> Affine3 {
        Affine3::new(self.r, self.t + v)
    }
}

impl Sub<F3> for Affine3 {
    type Output = Affine3;
    #[inline(always)]
    fn sub(self, v: F3) -> Affine3 {
        Affine3::new(self.r, self.t - v)
    }
}
