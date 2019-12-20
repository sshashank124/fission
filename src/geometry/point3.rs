use std::ops::{Add, Deref, Div, Mul, Sub};

use super::*;
use crate::op;

#[derive(Clone, Copy)]
pub struct P(pub F3);

impl Zero for P {
    const ZERO: P = P(F3::ZERO);
}

impl P {
    #[inline(always)]
    pub fn a2(a: F2, z: F) -> P { P(A3::a2(a, z)) }
}

op!(Add::add, *P -> *P -> P);
op!(Add::add, *P -> *V -> P);
op!(Add::add, *P ->  F -> P);
op!(Sub::sub, *P -> *P -> V);
op!(Sub::sub, *P -> *V -> P);
op!(Sub::sub, *P ->  F -> P);
op!(Mul::mul, *P ->  F -> P);
op!(Mul::mul,  T -> *P -> P);
op!(Div::div,  T -> *P -> P);

impl Deref for P {
    type Target = F3;
    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.0 }
}
