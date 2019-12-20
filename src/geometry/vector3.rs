use std::ops::{Add, Deref, Div, Mul, Neg, Sub};

use super::*;
use crate::op;

#[derive(Clone, Copy, Debug)]
pub struct V(pub F3);

impl Zero for V {
    const ZERO: Self = V(F3::ZERO);
}

impl V {
    #[inline(always)]
    pub fn v(x: F, y: F, z: F) -> V { V(A3(x, y, z)) }

    #[inline(always)]
    pub fn a2(a: F2, z: F) -> V { V(A3::a2(a, z)) }

    #[inline(always)]
    pub fn dot(self, v: V) -> F { A3::dot(*self, *v) }
    #[inline(always)]
    pub fn norm2(self) -> F { self.dot(self) }
    #[inline(always)]
    pub fn norm(self) -> F { self.norm2().sqrt() }
    #[inline(always)]
    pub fn unit(self) -> V { self / self.norm() }

    #[inline(always)]
    pub fn shiftl(self) -> V { V::v(self[Y], self[Z], self[X]) }
    #[inline(always)]
    pub fn shiftr(self) -> V { V::v(self[Z], self[X], self[Y]) }

    #[inline(always)]
    pub fn cross(self, v: V) -> V {
        V(self.shiftl().zip(*v.shiftr(), Mul::mul)
          - self.shiftr().zip(*v.shiftl(), Mul::mul))
    }
}

op!(Neg::neg, *V);
op!(Add::add, *V -> *V -> V);
op!(Sub::sub, *V -> *V -> V);
op!(Mul::mul, *V ->  F -> V);
op!(Div::div, *V ->  F -> V);

impl Mul<V> for T {
    type Output = V;
    #[inline(always)]
    fn mul(self, v: V) -> V { V(self.rot() * *v) }
}

impl Div<V> for T {
    type Output = V;
    #[inline(always)]
    fn div(self, v: V) -> V { V(self.rot() / *v) }
}

impl Deref for V {
    type Target = F3;
    #[inline(always)]
    fn deref(&self) -> &F3 { &self.0 }
}
