use std::ops::{Add, Sub, Mul};

use super::*;
use rotscale3::RotScale3;


#[derive(Clone, Copy, Debug)]
pub struct Affine3 {
    r: RotScale3,
    t: F3,
}

impl One for Affine3 {
    const ONE: Affine3 = Affine3::new(RotScale3::ONE, F3::ZERO);
}

impl Affine3 {
    #[inline(always)]
    pub const fn new(r: RotScale3, t: F3) -> Affine3 { Affine3 { r, t } }

    #[inline(always)]
    pub fn from_cols(c1: F3, c2: F3, c3: F3, c4: F3) -> Affine3 {
        Affine3::new(RotScale3::from_cols(c1, c2, c3), c4)
    }

    #[inline(always)]
    pub fn scale(v: F3) -> Affine3 {
        Affine3::new(RotScale3::scale(v), F3::ZERO)
    }

    #[inline(always)]
    pub fn tr(self) -> Affine3 { Affine3::new(self.r.tr(), F3::ZERO) }

    #[inline(always)]
    pub fn rot(self) -> Affine3 { Affine3::new(self.r, F3::ZERO) }
}

impl Mul for Affine3 {
    type Output = Affine3;
    #[inline(always)]
    fn mul(self, m: Affine3) -> Affine3 {
        Affine3::new(self.r * m.r, self * m.t)
    }
}

impl<B, C> Mul<A3<B>> for Affine3 where B: Copy + Mul<F, Output=C>,
                                        C: Add<C, Output=C>,
                                        C: Add<F, Output=C> {
    type Output = A3<C>;
    #[inline(always)]
    fn mul(self, o: A3<B>) -> A3<C> { zip(self.r * o, self.t, Add::add) }
}

impl Add<F3> for Affine3 {
    type Output = Affine3;
    #[inline(always)]
    fn add(self, v: F3) -> Affine3 { Affine3::new(self.r, self.t + v) }
}

impl Sub<F3> for Affine3 {
    type Output = Affine3;
    #[inline(always)]
    fn sub(self, v: F3) -> Affine3 { Affine3::new(self.r, self.t - v) }
}
