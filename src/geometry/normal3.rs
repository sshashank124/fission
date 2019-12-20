use std::ops::{Add, Deref, Div, Mul};

use super::*;
use crate::op;

#[derive(Clone, Copy)]
pub struct N(V);

impl Zero for N {
    const ZERO: Self = N(V::ZERO);
}

impl N {
    #[inline(always)]
    pub fn v(v: V) -> N { N(v.unit()) }
    #[inline(always)]
    pub fn a3(v: F3) -> N { N::v(V(v)) }
}

op!(Add::add, *N -> *N -> N);
op!(Mul::mul, *N ->  F -> N);

impl Mul<N> for T {
    type Output = N;
    #[inline(always)]
    fn mul(self, n: N) -> N { N::v(self.inv().t() * *n) }
}

impl Div<N> for T {
    type Output = N;
    #[inline(always)]
    fn div(self, n: N) -> N { N::v(self.inv().t() / *n) }
}

impl Deref for N {
    type Target = V;
    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.0 }
}
