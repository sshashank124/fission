use std::cmp::min;
use std::ops::{Add, Sub, Mul, Div};

use super::*;


#[derive(Clone, Copy, Debug)]
pub struct P2<S>(pub S, pub S);

impl I2 {
    pub const ZERO: I2 = P2(0, 0);

    #[inline(always)]
    pub fn cw_min(self, P2(x, y): I2) -> I2 {
        P2(min(self.0, x), min(self.1, y))
    }
}

impl Add for I2 {
    type Output = I2;
    #[inline(always)]
    fn add(self, P2(x, y): I2) -> I2 {
        P2(self.0 + x, self.1 + y)
    }
}

impl Add<F2> for I2 {
    type Output = F2;
    #[inline(always)]
    fn add(self, P2(x, y): F2) -> F2 {
        P2(self.0 as F + x, self.1 as F + y)
    }
}

impl Sub for I2 {
    type Output = I2;
    #[inline(always)]
    fn sub(self, P2(x, y): I2) -> I2 {
        P2(self.0 - x, self.1 - y)
    }
}

impl Sub<I> for I2 {
    type Output = I2;
    #[inline(always)]
    fn sub(self, i: I) -> I2 {
        P2(self.0 - i, self.1 - i)
    }
}

impl Mul for I2 {
    type Output = I2;
    #[inline(always)]
    fn mul(self, P2(x, y): I2) -> I2 {
        P2(self.0 * x, self.1 * y)
    }
}

impl Mul<I2> for I {
    type Output = I2;
    #[inline(always)]
    fn mul(self, P2(x, y): I2) -> I2 {
        P2(self * x, self * y)
    }
}

impl Mul<I2> for F {
    type Output = F2;
    #[inline(always)]
    fn mul(self, P2(x, y): I2) -> F2 {
        P2(self * x as F, self * y as F)
    }
}

impl Mul<F2> for F {
    type Output = F2;
    #[inline(always)]
    fn mul(self, P2(x, y): F2) -> F2 {
        P2(self * x, self * y)
    }
}

impl Div for I2 {
    type Output = I2;
    #[inline(always)]
    fn div(self, P2(x, y): I2) -> I2 {
        P2(self.0 / x, self.1 / y)
    }
}

impl Div<F> for I2 {
    type Output = F2;
    #[inline(always)]
    fn div(self, f: F) -> F2 {
        f.inv() * self
    }
}

impl Div<F> for F2 {
    type Output = F2;
    #[inline(always)]
    fn div(self, f: F) -> F2 {
        f.inv() * self
    }
}

impl Div<I> for F2 {
    type Output = F2;
    #[inline(always)]
    fn div(self, i: I) -> F2 {
        self / i as F
    }
}
