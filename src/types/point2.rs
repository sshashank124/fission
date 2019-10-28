use std::ops::{Add, Mul, Div};

use super::*;


#[derive(Clone, Copy, Debug)]
pub struct P2<S>(pub S, pub S);

impl Add<F2> for I2 {
    type Output = F2;
    #[inline]
    fn add(self, P2(x, y): F2) -> F2 {
        P2(self.0 as F + x, self.1 as F + y)
    }
}

impl Mul<I2> for F {
    type Output = F2;
    #[inline]
    fn mul(self, P2(x, y): I2) -> F2 {
        P2(self * x as F, self * y as F)
    }
}

impl Mul<F2> for F {
    type Output = F2;
    #[inline]
    fn mul(self, P2(x, y): F2) -> F2 {
        P2(self * x, self * y)
    }
}

impl Div<F> for I2 {
    type Output = F2;
    #[inline]
    fn div(self, f: F) -> F2 {
        f.inv() * self
    }
}

impl Div<F> for F2 {
    type Output = F2;
    #[inline]
    fn div(self, f: F) -> F2 {
        f.inv() * self
    }
}

impl Div<I> for F2 {
    type Output = F2;
    #[inline]
    fn div(self, i: I) -> F2 {
        self / i as F
    }
}
