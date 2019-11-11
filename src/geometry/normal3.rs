use std::ops::{Add, Mul, Div, Deref};

use super::*;
use crate::op;


#[derive(Clone, Copy, Debug)]
pub struct N(pub V);

impl Zero for N { const ZERO: Self = N(V::ZERO); }

impl N {
    #[inline(always)]
    pub fn unitn(self) -> N {
        N(self.unit())
    }
}

op!(Add::add, *N -> *N -> N);
op!(Mul::mul, *N ->  F -> N);

impl Mul<N> for T {
    type Output = N;
    #[inline(always)] fn mul(self, N(v): N) -> N { N(self.inv().tr() * v) }
}

impl Div<N> for T {
    type Output = N;
    #[inline(always)] fn div(self, N(v): N) -> N { N(self.inv().tr() / v) }
}

impl Deref for N {
    type Target = V;
    #[inline(always)] fn deref(&self) -> &Self::Target { &self.0 }
}
