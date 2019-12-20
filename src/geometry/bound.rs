use std::ops::{Add, BitAnd, BitOr, Deref, Div, Mul, Sub};

use super::*;
use crate::op;

#[derive(Clone, Copy)]
pub struct B(pub F2);

impl Zero for B {
    const ZERO: Self = B::b(F::POS_INF, F::NEG_INF);
}

impl B {
    #[inline(always)]
    pub const fn b(l: F, u: F) -> B { B(A2(l, u)) }
    #[inline(always)]
    pub fn point(f: F) -> B { B(A2::rep(f)) }

    #[inline(always)]
    pub fn ordered(a: F2) -> B {
        if a[0] > a[1] {
            B::b(a[1], a[0])
        } else {
            B::b(a[0], a[1])
        }
    }

    #[inline(always)]
    pub fn bounds(self, t: F) -> bool { self[0] <= t && t <= self[1] }

    #[inline(always)]
    pub fn degen(self) -> bool { self[0] > self[1] }

    pub fn center(self) -> F { self.mean() }
    pub fn extent(self) -> F { self[1] - self[0] }
}

op!(Add::add, *B -> *B -> B);
op!(Add::add, *B ->  F -> B);
op!(Sub::sub, *B ->  F -> B);

impl Mul<F> for B {
    type Output = B;
    #[inline(always)]
    fn mul(self, f: F) -> B { B::ordered(*self * f) }
}

impl Div<F> for B {
    type Output = B;
    #[inline(always)]
    fn div(self, f: F) -> B { self * f.inv() }
}

impl BitOr for B {
    type Output = B;
    #[inline(always)]
    fn bitor(self, b: B) -> B {
        B::b(F::min(self[0], b[0]), F::max(self[1], b[1]))
    }
}

impl BitOr<F> for B {
    type Output = B;
    #[inline(always)]
    fn bitor(self, f: F) -> B { self | B::point(f) }
}

impl BitAnd for B {
    type Output = B;
    #[inline(always)]
    fn bitand(self, b: B) -> B {
        B::b(F::max(self[0], b[0]), F::min(self[1], b[1]))
    }
}

impl Deref for B {
    type Target = F2;
    #[inline(always)]
    fn deref(&self) -> &F2 { &self.0 }
}
