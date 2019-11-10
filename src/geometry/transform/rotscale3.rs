use std::ops::{Add, Mul};

use super::*;


#[derive(Clone, Copy, Debug)]
pub struct RotScale3(A3<F3>);

impl RotScale3 {
    #[inline(always)]
    pub fn from_rows(r1: F3, r2: F3, r3: F3) -> RotScale3 {
        RotScale3(A3(r1, r2, r3))
    }

    #[inline(always)]
    pub fn from_cols(c1: F3, c2: F3, c3: F3) -> RotScale3 {
        RotScale3::from_rows(c1, c2, c3).t()
    }

    #[inline(always)]
    pub fn from_diag(d: F3) -> RotScale3 {
        RotScale3(A3(d.0 * F3::X, d.1 * F3::Y, d.2 * F3::Z))
    }

    #[inline(always)]
    pub fn scale(v: F3) -> RotScale3 {
        RotScale3::from_diag(v)
    }

    #[inline(always)]
    pub fn t(self) -> RotScale3 {
        RotScale3(self.0.t())
    }

    pub const I: RotScale3 = RotScale3(A3(F3::X, F3::Y, F3::Z));
}

impl Mul for RotScale3 {
    type Output = RotScale3;
    #[inline(always)]
    fn mul(self, m: RotScale3) -> RotScale3 {
        RotScale3(self * m.0)
    }
}

impl<B, Z> Mul<A3<B>> for RotScale3 where B: Copy + Mul<F, Output=Z>,
                                          Z: Add<Z, Output=Z> {
    type Output = A3<Z>;
    #[inline(always)]
    fn mul(self, o: A3<B>) -> A3<Z> {
        zip(rep(o), self.0, dot)
    }
}
