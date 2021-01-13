use std::ops::Div;

#[allow(clippy::wildcard_imports)]
use graphite::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct PDF<A> {
    pub val: A,
    pub pdf: F,
}

impl<A> PDF<A> {
    #[inline] pub const fn new(val: A, pdf: F) -> Self { Self { val, pdf } }
    #[inline] pub const fn sole(val: A) -> Self { Self::new(val, 1.) }

}

impl<A> PDF<A> where A: Div<F, Output=A> {
    #[inline] pub fn scale(self, s: F) -> Self { Self::new(self.val / s, self.pdf) }
}

impl<A> Zero for PDF<A> where A: Zero { const ZERO: Self = Self::new(A::ZERO, F::ZERO); }
