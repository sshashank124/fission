use std::ops::{Mul, Div};

use super::*;


pub trait Intersectable {
    fn bbox(&self, t: T) -> BBox;
    fn intersects(&self, ray: R) -> bool;
    fn intersect(&self, ray: R) -> Option<Its>;
}

#[derive(Clone, Copy)]
pub struct Its {
    pub p:  P,
    pub t:  F,
    pub n:  N,
}

impl Its {
    #[inline(always)]
    pub fn new(p: P, t: F, n: N) -> Its {
        Its { p, t, n }
    }
}

impl Mul<Its> for T {
    type Output = Its;
    #[inline(always)]
    fn mul(self, Its{p, t, n}: Its) -> Its {
        Its::new(self * p, t, self * n)
    }
}

impl Div<Its> for T {
    type Output = Its;
    #[inline(always)]
    fn div(self, Its{p, t, n}: Its) -> Its {
        Its::new(self / p, t, self / n)
    }
}
