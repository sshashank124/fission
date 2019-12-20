mod affine3;
mod normalization2;
mod rotscale3;

use std::ops::{Add, Div, Mul};

use super::*;

pub use normalization2::Norm2 as T2;
pub use TransformPair3 as T;

type T3 = affine3::Affine3;

#[derive(Clone, Copy, Debug)]
pub struct TransformPair3 {
    f: T3,
    i: T3,
}

impl One for TransformPair3 {
    const ONE: Self = Self::new(T3::ONE, T3::ONE);
}

impl TransformPair3 {
    #[inline(always)]
    const fn new(f: T3, i: T3) -> Self { Self { f, i } }

    #[inline(always)]
    pub fn translate(v: F3) -> Self {
        Self::new(T3::translate(v), T3::translate(-v))
    }

    #[inline(always)]
    pub fn scale(v: F3) -> Self { Self::new(T3::scale(v), T3::scale(v.inv())) }

    #[inline(always)]
    pub fn rotate(axis: F3, theta: F) -> Self {
        Self::new(T3::rotate(axis, theta), T3::rotate(axis, -theta))
    }

    #[inline(always)]
    pub fn from_frame(v: V) -> Self {
        let t = T3::from_frame(v);
        Self::new(t, t.t())
    }

    #[inline(always)]
    pub fn look_at(pos: P, target: P, up: V) -> Self {
        Self::new(T3::look_at(pos, target, up), T3::ONE)
    }

    #[inline(always)]
    pub fn rot(&self) -> Self { Self::new(self.f.rot(), self.i.rot()) }

    #[inline(always)]
    pub fn t(&self) -> Self { Self::new(self.f.t(), self.i.t()) }
}

impl Inv for TransformPair3 {
    type Output = Self;
    #[inline(always)]
    fn inv(self) -> Self { Self::new(self.i, self.f) }
}

impl Mul for TransformPair3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, s: Self) -> Self { Self::new(self.f * s.f, s.i * self.i) }
}

impl<B> Mul<A3<B>> for TransformPair3
    where B: Copy + Mul<F, Output = B> + Add<Output = B> + Add<F, Output = B>
{
    type Output = A3<B>;
    #[inline(always)]
    fn mul(self, t: A3<B>) -> A3<B> { self.f * t }
}

impl<B> Div<A3<B>> for TransformPair3
    where B: Copy + Mul<F, Output = B> + Add<Output = B> + Add<F, Output = B>
{
    type Output = A3<B>;
    #[inline(always)]
    fn div(self, v: A3<B>) -> A3<B> { self.i * v }
}
