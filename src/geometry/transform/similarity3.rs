use std::ops::{Add, Mul, Div};

use super::*;
use affine3::Affine3;


#[derive(Clone, Copy)]
pub struct Sim3 {
    f: Affine3,
    i: Affine3,
}

impl Sim3 {
    #[inline(always)]
    pub const fn new(f: Affine3, i: Affine3) -> Sim3 {
        Sim3 { f, i }
    }

    #[inline(always)]
    pub fn translate(v: F3) -> Sim3 {
        Sim3::new(Affine3::I + v, Affine3::I - v)
    }

    #[inline(always)]
    pub fn inv(self) -> Sim3 {
        Sim3::new(self.i, self.f)
    }

    #[inline(always)]
    pub fn t(self) -> Sim3 {
        Sim3::new(self.f.t(), self.i.t())
    }

    #[inline(always)]
    pub fn rot(self) -> Sim3 {
        Sim3::new(self.f.rot(), self.i.rot())
    }

    pub const I: Sim3 = Sim3::new(Affine3::I, Affine3::I);
}

impl Mul for Sim3 {
    type Output = Sim3;
    #[inline(always)]
    fn mul(self, s: Sim3) -> Sim3 {
        Sim3::new(self.f * s.f, s.i * self.i)
    }
}

impl<B, Z> Mul<A3<B>> for Sim3 where B: Copy + Mul<F, Output=Z>,
                                     Z: Add<Z, Output=Z>,
                                     Z: Add<F, Output=Z> {
    type Output = A3<Z>;
    #[inline(always)]
    fn mul(self, t: A3<B>) -> A3<Z> {
        self.f * t
    }
}

impl<B, Z> Div<A3<B>> for Sim3 where B: Copy + Mul<F, Output=Z>,
                                     Z: Add<Z, Output=Z>,
                                     Z: Add<F, Output=Z> {
    type Output = A3<Z>;
    #[inline(always)]
    fn div(self, v: A3<B>) -> A3<Z> {
        self.i * v
    }
}
