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
    pub ng: N,
}

impl Its {
    #[inline(always)]
    pub fn new(p: P, t: F, n: N, ng: N) -> Its {
        Its { p, t, n, ng }
    }

    #[inline(always)]
    pub fn ideal(p: P, t: F, n: N) -> Its {
        Its::new(p, t, n, n)
    }
}

impl Mul<Its> for T {
    type Output = Its;
    #[inline(always)]
    fn mul(self, Its{p, t, n, ng}: Its) -> Its {
        Its::new(self * p, t, self * n, self * ng)
    }
}

impl Div<Its> for T {
    type Output = Its;
    #[inline(always)]
    fn div(self, Its{p, t, n, ng}: Its) -> Its {
        Its::new(self / p, t, self / n, self / ng)
    }
}
