use std::ops::{Add, Sub, Mul, Div};

use super::*;


#[derive(Clone, Copy, Debug)]
pub struct P(pub F3);

impl P {
    #[inline]
    pub const fn x(&self) -> F {
        (self.0).0
    }

    #[inline]
    pub const fn y(&self) -> F {
        (self.0).1
    }

    #[inline]
    pub const fn z(&self) -> F {
        (self.0).2
    }

    pub const ZERO: P = P(F3::ZERO);
}

impl Add for P {
    type Output = P;
    #[inline]
    fn add(self, p: P) -> P {
        P(self.0 + p.0)
    }
}

impl Add<V> for P {
    type Output = P;
    #[inline]
    fn add(self, v: V) -> P {
        P(self.0 + v.0)
    }
}

impl Sub<V> for P {
    type Output = P;
    #[inline]
    fn sub(self, v: V) -> P {
        P(self.0 - v.0)
    }
}

impl Sub for P {
    type Output = V;
    #[inline]
    fn sub(self, p: P) -> V {
        V(self.0 - p.0)
    }
}

impl Mul<P> for T {
    type Output = P;
    #[inline]
    fn mul(self, P(p): P) -> P {
        P(self * p)
    }
}

impl Div<P> for T {
    type Output = P;
    #[inline]
    fn div(self, P(p): P) -> P {
        P(self / p)
    }
}
