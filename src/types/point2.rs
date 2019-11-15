use std::cmp::min;
use std::ops::{Add, Sub, Mul, Div, Index, IndexMut};

use super::*;


#[derive(Clone, Copy, Debug)]
pub struct P2<S>(pub S, pub S);

impl<S> Zero for P2<S> where S: Zero {
    const ZERO: Self = P2(S::ZERO, S::ZERO);
}

impl F2 { pub const HALF: F2 = P2(0.5, 0.5); }

impl I2 {
    #[inline(always)]
    pub fn cw_min(self, P2(x, y): I2) -> I2 {
        P2(min(self[X], x), min(self[Y], y))
    }
}

impl Add for I2 {
    type Output = I2;
    #[inline(always)]
    fn add(self, P2(x, y): I2) -> I2 {
        P2(self[X] + x, self[Y] + y)
    }
}

impl Add<F> for F2 {
    type Output = F2;
    #[inline(always)]
    fn add(self, f: F) -> F2 {
        P2(self[X] + f, self[Y] + f)
    }
}


impl Add<F2> for I2 {
    type Output = F2;
    #[inline(always)]
    fn add(self, P2(x, y): F2) -> F2 {
        P2(self[X] as F + x, self[Y] as F + y)
    }
}

impl Sub for I2 {
    type Output = I2;
    #[inline(always)]
    fn sub(self, P2(x, y): I2) -> I2 {
        P2(self[X] - x, self[Y] - y)
    }
}

impl Sub for F2 {
    type Output = F2;
    #[inline(always)]
    fn sub(self, P2(x, y): F2) -> F2 {
        P2(self[X] - x, self[Y] - y)
    }
}

impl Sub<I2> for F2 {
    type Output = F2;
    #[inline(always)]
    fn sub(self, P2(x, y): I2) -> F2 {
        P2(self[X] - x as F, self[Y] - y as F)
    }
}

impl Sub<F> for F2 {
    type Output = F2;
    #[inline(always)]
    fn sub(self, f: F) -> F2 {
        P2(self[X] - f, self[Y] - f)
    }
}

impl Sub<I> for I2 {
    type Output = I2;
    #[inline(always)]
    fn sub(self, i: I) -> I2 {
        P2(self[X] - i, self[Y] - i)
    }
}

impl Mul for I2 {
    type Output = I2;
    #[inline(always)]
    fn mul(self, P2(x, y): I2) -> I2 {
        P2(self[X] * x, self[Y] * y)
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
        P2(self[X] / x, self[Y] / y)
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

impl<S> Index<Dim> for P2<S> {
    type Output = S;
    #[inline(always)]
    fn index(&self, dim: Dim) -> &S {
        match dim {
            X => &self.0,
            Y => &self.1,
            _ => unreachable!(),
        }
    }
}

impl<S> Index<I> for P2<S> {
    type Output = S;
    #[inline(always)]
    fn index(&self, i: I) -> &S {
        match i {
            0 => &self.0,
            1 => &self.1,
            _ => unreachable!(),
        }
    }
}

impl<S> IndexMut<Dim> for P2<S> {
    #[inline(always)]
    fn index_mut(&mut self, dim: Dim) -> &mut S {
        match dim {
            X => &mut self.0,
            Y => &mut self.1,
            _ => unreachable!(),
        }
    }
}
