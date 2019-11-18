use std::ops::{Mul, Div};

use super::*;


#[derive(Clone, Copy)]
pub struct R {
    pub o: P,
    pub d: V,
    pub t: F,
}

impl R {
    #[inline(always)]
    pub fn r(o: P, d: V, t: F) -> R { R { o, d, t } }

    #[inline(always)]
    pub fn unbounded(o: P, d: V) -> R { R::r(o, d, F::POS_INF) }

    #[inline(always)]
    pub fn at(&self, t: F) -> P { self.o + self.d * t }

    #[inline(always)]
    pub fn clipped(self, t: F) -> R { R::r(self.o, self.d, t) }

    #[inline(always)]
    pub fn range(&self) -> B { B::b(F::EPSILON, self.t) }
}

impl Mul<R> for T { type Output = R;
    #[inline(always)]
    fn mul(self, R{o, d, t}: R) -> R { R::r(self * o, self * d, t) }
}

impl Div<R> for T { type Output = R;
    #[inline(always)]
    fn div(self, R{o, d, t}: R) -> R { R::r(self / o, self / d, t) }
}
