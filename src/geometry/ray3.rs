use std::ops::{Mul, Div};

use super::*;


#[derive(Clone, Copy, Debug)]
pub struct R {
    pub o:  P,
    pub d:  V,
    pub d_inv: V,
    pub tb: B,
}

impl R {
    #[inline(always)]
    pub fn r(o: P, d: V, d_inv: V, tb: B) -> R { R { o, d, d_inv, tb } }

    #[inline(always)]
    pub fn new(o: P, d: V, tb: B) -> R { R::r(o, d, V(d.inv()), tb) }

    #[inline(always)]
    pub fn unbounded(o: P, d: V) -> R { R::new(o, d, B::POSITIVE) }

    #[inline(always)]
    pub fn at(&self, t: F) -> P { self.o + self.d * t }

    #[inline(always)] pub fn clip(&mut self, t: F) { self.tb.set_upper(t) }
}

impl Mul<R> for T {
    type Output = R;
    #[inline(always)]
    fn mul(self, R{o, d, d_inv: _, tb}: R) -> R {
        R::new(self * o, self * d, tb)
    }
}

impl Div<R> for T {
    type Output = R;
    #[inline(always)]
    fn div(self, R{o, d, d_inv: _, tb}: R) -> R {
        R::new(self / o, self / d, tb)
    }
}
