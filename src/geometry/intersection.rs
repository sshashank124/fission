use std::ops::{Mul, Div};

use super::*;


pub trait Intersectable {
    fn bbox(&self, t: T) -> BBox;
    fn intersects(&self, ray: &R) -> bool;
    fn intersect(&self, ray: &mut R) -> Option<Its>;
    fn hit_info(&self, its: &mut Its);
}

pub struct Its {
    pub p: P,
    pub n: N,
    pub uv: F2,
}

impl Its {
    #[inline(always)]
    pub fn new(p: P, n: N, uv: F2) -> Its {
        Its { p, n, uv }
    }
}

impl Mul<Its> for T {
    type Output = Its;
    #[inline(always)]
    fn mul(self, Its{p, n, uv}: Its) -> Its {
        Its::new(self * p, self * n, uv)
    }
}

impl Div<Its> for T {
    type Output = Its;
    #[inline(always)]
    fn div(self, Its{p, n, uv}: Its) -> Its {
        Its::new(self / p, self / n, uv)
    }
}
