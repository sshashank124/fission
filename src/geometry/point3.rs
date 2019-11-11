use std::ops::{Add, Sub, Mul, Div, Deref};

use super::*;
use crate::op;


#[derive(Clone, Copy, Debug)]
pub struct P(pub F3);

impl Zero for P { const ZERO: Self = P(F3::ZERO); }

impl P {
    #[inline(always)]
    pub fn p(x: F, y: F, z: F) -> P { P(A3(x, y, z)) }
}

op!(Add::add, *P -> *P -> P);
op!(Add::add, *P -> *V -> P);
op!(Sub::sub, *P -> *V -> P);
op!(Sub::sub, *P -> *P -> V);
op!(Mul::mul, *P ->  F -> P);
op!(Mul::mul,  T -> *P -> P);
op!(Div::div,  T -> *P -> P);

impl Deref for P {
    type Target = F3;
    #[inline(always)] fn deref(&self) -> &Self::Target { &self.0 }
}
