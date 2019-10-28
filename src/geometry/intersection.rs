use std::ops::{Mul, Div};

use super::*;


pub struct Its {
    pub p:  P,
    pub t:  F,
    pub n:  N,
    pub ng: N,
}

impl Its {
    #[inline]
    pub fn new(p: P, t: F, n: N, ng: N) -> Its {
        Its { p, t, n, ng }
    }

    #[inline]
    pub fn ideal(p: P, t: F, n: N) -> Its {
        Its::new(p, t, n, n)
    }
}

impl Mul<Its> for T {
    type Output = Its;
    #[inline]
    fn mul(self, Its{p, t, n, ng}: Its) -> Its {
        Its::new(self * p, t, self * n, self * ng)
    }
}

impl Div<Its> for T {
    type Output = Its;
    #[inline]
    fn div(self, Its{p, t, n, ng}: Its) -> Its {
        Its::new(self / p, t, self / n, self / ng)
    }
}
