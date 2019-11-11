use std::ops::{Add, Mul, Deref};

use super::*;


#[derive(Clone, Copy, Debug)]
pub struct RotScale3(A3<F3>);

impl One for RotScale3 {
    const ONE: Self = RotScale3(A3(F3::X, F3::Y, F3::Z));
}

impl RotScale3 {
    #[inline(always)]
    pub fn from_rows(r1: F3, r2: F3, r3: F3) -> RotScale3 {
        RotScale3(A3(r1, r2, r3))
    }

    #[inline(always)]
    pub fn from_cols(c1: F3, c2: F3, c3: F3) -> RotScale3 {
        RotScale3::from_rows(c1, c2, c3).tr()
    }

    #[inline(always)]
    pub fn from_diag(d: F3) -> RotScale3 {
        RotScale3(zip(*Self::ONE, d, Mul::mul))
    }

    #[inline(always)]
    pub fn scale(v: F3) -> RotScale3 { RotScale3::from_diag(v) }

    #[inline(always)] pub fn tr(self) -> RotScale3 { RotScale3(self.t()) }
}

impl Mul for RotScale3 {
    type Output = RotScale3;
    #[inline(always)]
    fn mul(self, m: RotScale3) -> RotScale3 {
        RotScale3(self * *m)
    }
}

impl<B, C> Mul<A3<B>> for RotScale3 where B: Copy + Mul<F, Output=C>,
                                          C: Add<C, Output=C> {
    type Output = A3<C>;
    #[inline(always)]
    fn mul(self, o: A3<B>) -> A3<C> {
        zip(rep(o), *self, dot)
    }
}

impl Deref for RotScale3 {
    type Target = A3<F3>;
    #[inline(always)] fn deref(&self) -> &Self::Target { &self.0 }
}
