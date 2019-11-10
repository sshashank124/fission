use std::ops::{Add, BitAnd, BitOr, Sub, Mul, Div};

use super::*;


#[derive(Clone, Copy, Debug)]
pub struct B(pub F, pub F);

impl B {
    #[inline(always)]
    pub fn point(f: F) -> B {
        B(f, f)
    }

    #[inline(always)]
    pub fn ordered(a: F, b: F) -> B {
        if a < b { B(a, b) } else { B(b, a) }
    }

    #[inline(always)]
    pub fn with_ceil(u: F) -> B {
        B(F::EPSILON, u)
    }

    #[inline(always)]
    pub fn with_upper(self, u: F) -> B {
        B(self.0, u)
    }

    #[inline(always)]
    pub fn bounds(self, f: F) -> bool {
        self.0 <= f && f <= self.1
    }

    #[inline(always)]
    pub fn degen(self) -> bool {
        self.0 > self.1
    }

    #[inline(always)]
    pub fn overlaps(self, b: B) -> bool {
        self.0 < b.1 && b.0 < self.1
    }

    #[inline(always)]
    pub fn center(self) -> F {
        0.5 * self.0 + 0.5 * self.1
    }

    #[inline(always)]
    pub fn extent(self) -> F {
        self.1 - self.0
    }

    pub const EMPTY:    B = B(F::POS_INF, F::NEG_INF);
    pub const INF:      B = B(F::NEG_INF, F::POS_INF);
    pub const POSITIVE: B = B(F::EPSILON, F::POS_INF);
}

impl Add for B {
    type Output = B;
    #[inline(always)]
    fn add(self, B(l, u): B) -> B {
        B(self.0 + l, self.1 + u)
    }
}

impl Add<F> for B {
    type Output = B;
    #[inline(always)]
    fn add(self, f: F) -> B {
        B(self.0 + f, self.1 + f)
    }
}

impl Sub<F> for B {
    type Output = B;
    #[inline(always)]
    fn sub(self, f: F) -> B {
        B(self.0 - f, self.1 - f)
    }
}

impl Mul<F> for B {
    type Output = B;
    #[inline(always)]
    fn mul(self, f: F) -> B {
        B::ordered(self.0 * f, self.1 * f)
    }
}

impl Mul<B> for F {
    type Output = B;
    #[inline(always)]
    fn mul(self, b: B) -> B {
        b * self
    }
}

impl Div<F> for B {
    type Output = B;
    #[inline(always)]
    fn div(self, f: F) -> B {
        self * f.inv()
    }
}

// Union
impl BitOr for B {
    type Output = B;
    #[inline(always)]
    fn bitor(self, B(l, u): B) -> B {
        B(F::min(self.0, l), F::max(self.1, u))
    }
}

impl BitOr<F> for B {
    type Output = B;
    #[inline(always)]
    fn bitor(self, f: F) -> B {
        self | B::point(f)
    }
}

// Intersection
impl BitAnd for B {
    type Output = B;
    #[inline(always)]
    fn bitand(self, B(l, u): B) -> B {
        B(F::max(self.0, l), F::min(self.1, u))
    }
}
