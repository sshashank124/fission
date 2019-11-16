use std::ops::{Add, Mul, Deref};

use super::*;


#[derive(Clone, Copy)]
pub struct RotScale3(A3<F3>);

impl One for RotScale3 {
    const ONE: Self = Self(A3(F3::X, F3::Y, F3::Z));
}

impl RotScale3 {
    #[inline(always)]
    pub fn from_rows(r1: F3, r2: F3, r3: F3) -> Self { Self(A3(r1, r2, r3)) }

    #[inline(always)]
    pub fn from_cols(c1: F3, c2: F3, c3: F3) -> Self {
        Self::from_rows(c1, c2, c3).t()
    }

    #[inline(always)]
    pub fn from_diag(d: F3) -> Self { Self(Self::ONE.zip(d, Mul::mul)) }

    #[inline(always)]
    pub fn scale(v: F3) -> Self { Self::from_diag(v) }

    #[inline(always)]
    pub fn rotate(axis: F3, theta: F) -> Self {
        // TODO refactor into more abstract operations
        let V(A3(x, y, z)) = V(axis).unit();
        let ct = theta.cosd(); let cc = 1. - ct; let st = theta.sind();
        Self::from_rows(A3(ct+x.sq()*cc, x*y*cc-z*st, x*z*cc+y*st),
                        A3(y*x*cc+z*st, ct+y.sq()*cc, y*z*cc-x*st),
                        A3(z*x*cc-y*st, z*y*cc+x*st, ct+z.sq()*cc))
    }

    #[inline(always)] pub fn t(&self) -> Self { Self(self.unzip(A3)) }
}

impl Mul for RotScale3 {
    type Output = Self;
    #[inline(always)] fn mul(self, m: Self) -> Self { Self(self * *m) }
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
