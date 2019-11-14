use std::ops::{Add, Sub, Mul, Div, Neg, Deref};

use super::*;
use crate::op;


#[derive(Clone, Copy)]
pub struct V(pub F3);

impl Zero for V { const ZERO: Self = V(F3::ZERO); }

impl V {
    #[inline(always)] pub fn v(x: F, y: F, z: F) -> V { V(A3(x, y, z)) }

    #[inline(always)] pub fn dot(self, V(v): V) -> F { dot(*self, v) }
    #[inline(always)] pub fn norm2(self) -> F { self.dot(self) }
    #[inline(always)] pub fn norm(self) -> F { self.norm2().sqrt() }
    #[inline(always)] pub fn unit(self) -> V { self / self.norm() }

    #[inline(always)]
    pub fn shiftl(self) -> V { let A3(x, y, z) = *self; V::v(y, z, x) }
    #[inline(always)]
    pub fn shiftr(self) -> V { let A3(x, y, z) = *self; V::v(z, x, y) }

    #[inline(always)]
    pub fn cross(self, v: V) -> V {
        V(zip(*self.shiftl(), *v.shiftr(), Mul::mul) -
          zip(*self.shiftr(), *v.shiftl(), Mul::mul))
    }
}

op!(Neg::neg, *V);
op!(Add::add, *V -> *V -> V);
op!(Sub::sub, *V -> *V -> V);
op!(Mul::mul, *V ->  F -> V);
op!(Div::div, *V ->  F -> V);

impl Mul<V> for T {
    type Output = V;
    #[inline(always)] fn mul(self, V(v): V) -> V { V(self.rot() * v) }
}

impl Div<V> for T {
    type Output = V;
    #[inline(always)] fn div(self, V(v): V) -> V { V(self.rot() / v) }
}

impl Deref for V {
    type Target = F3;
    #[inline(always)] fn deref(&self) -> &Self::Target { &self.0 }
}
