use std::ops::{Add, Mul, Div};

use super::*;
use affine3::Affine3;


#[derive(Clone, Copy)]
pub struct Sim3 {
    f: Affine3,
    i: Affine3,
}

impl One for Sim3 {
    const ONE: Self = Sim3::new(Affine3::ONE, Affine3::ONE);
}

impl Sim3 {
    #[inline(always)]
    pub const fn new(f: Affine3, i: Affine3) -> Sim3 { Sim3 { f, i } }

    #[inline(always)]
    pub fn translate(v: F3) -> Sim3 {
        Sim3::new(Affine3::ONE + v, Affine3::ONE - v)
    }

    #[inline(always)]
    pub fn scale(v: F3) -> Sim3 {
        Sim3::new(Affine3::scale(v), Affine3::scale(v.inv()))
    }

    #[inline(always)]
    pub fn from_dir(v: V) -> Sim3 {
        let dx0 = V(F3::X).cross(v);
        let dx1 = V(F3::Y).cross(v);
        let dx = if dx0.norm2() > dx1.norm2() { dx0 } else { dx1 }.unit();
        let dy = v.cross(dx).unit();
        Sim3::new(Affine3::from_cols(*dx, *dy, *v, F3::ZERO),
                  Affine3::ONE)  // maybe not needed?
    }

    #[inline(always)]
    pub fn look_at(pos: P, target: P, up: V) -> Sim3 {
        let dir = (target - pos).unit();
        let right = (up.unit().cross(dir)).unit();
        let up = (dir.cross(right)).unit();
        Sim3::new(Affine3::from_cols(*right, *up, *dir, *pos),
                  Affine3::ONE)  // should not need to be used
    }

    #[inline(always)] pub fn inv(self) -> Sim3 { Sim3::new(self.i, self.f) }

    #[inline(always)]
    pub fn tr(self) -> Sim3 { Sim3::new(self.f.tr(), self.i.tr()) }

    #[inline(always)]
    pub fn rot(self) -> Sim3 { Sim3::new(self.f.rot(), self.i.rot()) }
}

impl Mul for Sim3 {
    type Output = Sim3;
    #[inline(always)]
    fn mul(self, s: Sim3) -> Sim3 { Sim3::new(self.f * s.f, s.i * self.i) }
}

impl<B, C> Mul<A3<B>> for Sim3 where B: Copy + Mul<F, Output=C>,
                                     C: Add<C, Output=C>,
                                     C: Add<F, Output=C> {
    type Output = A3<C>;
    #[inline(always)]
    fn mul(self, t: A3<B>) -> A3<C> { self.f * t }
}

impl<B, C> Div<A3<B>> for Sim3 where B: Copy + Mul<F, Output=C>,
                                     C: Add<C, Output=C>,
                                     C: Add<F, Output=C> {
    type Output = A3<C>;
    #[inline(always)]
    fn div(self, v: A3<B>) -> A3<C> { self.i * v }
}
