use std::ops::{Add, Mul, Div};

use super::*;
use affine3::Affine3;


#[derive(Clone, Copy)]
pub struct Sim3 {
    f: Affine3,
    i: Affine3,
}

impl Sim3 {
    #[inline]
    pub const fn new(f: Affine3, i: Affine3) -> Sim3 {
        Sim3 { f, i }
    }

    #[inline]
    pub fn translate(v: F3) -> Sim3 {
        Sim3::new(Affine3::I + v, Affine3::I - v)
    }

    #[inline]
    pub fn inv(self) -> Sim3 {
        Sim3::new(self.i, self.f)
    }

    #[inline]
    pub fn t(self) -> Sim3 {
        Sim3::new(self.f.t(), self.i.t())
    }

    #[inline]
    pub fn rot(self) -> Sim3 {
        Sim3::new(self.f.rot(), self.i.rot())
    }

    pub const I: Sim3 = Sim3::new(Affine3::I, Affine3::I);
}

impl Mul for Sim3 {
    type Output = Sim3;
    #[inline]
    fn mul(self, s: Sim3) -> Sim3 {
        Sim3::new(self.f * s.f, s.i * self.i)
    }
}

impl<B, Z> Mul<A3<B>> for Sim3 where B: Copy + Mul<F, Output=Z>,
                                     Z: Add<Z, Output=Z>,
                                     Z: Add<F, Output=Z> {
    type Output = A3<Z>;
    #[inline]
    fn mul(self, t: A3<B>) -> A3<Z> {
        self.f * t
    }
}

impl<B, Z> Div<A3<B>> for Sim3 where B: Copy + Mul<F, Output=Z>,
                                     Z: Add<Z, Output=Z>,
                                     Z: Add<F, Output=Z> {
    type Output = A3<Z>;
    #[inline]
    fn div(self, v: A3<B>) -> A3<Z> {
        self.i * v
    }
}
