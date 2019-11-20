use std::ops::{Mul, Div};

use super::*;


pub struct Its {
    pub p: P,
    pub n: N,
    pub uv: F2,
    pub t: F,
    pub i: I,
}

impl Its {
    #[inline(always)] pub fn new(p: P, n: N, uv: F2, t: F, i: I) -> Its
    { Its { p, n, uv, t, i } }
}

impl Mul<Its> for T { type Output = Its;
    #[inline(always)] fn mul(self, Its{p, n, uv, t, i}: Its) -> Its
    { Its::new(self * p, self * n, uv, t, i) }
}

impl Div<Its> for T { type Output = Its;
    #[inline(always)] fn div(self, Its{p, n, uv, t, i}: Its) -> Its
    { Its::new(self / p, self / n, uv, t, i) }
}
