use std::ops::{Add, AddAssign, Sub, Mul, Div, DivAssign, Neg};

use super::*;


impl<FT> A3<FT> where FT: Float {
    pub const ZERO: A3<FT> = A3(FT::ZERO, FT::ZERO, FT::ZERO);
    pub const X:    A3<FT> = A3(FT::ONE,  FT::ZERO, FT::ZERO);
    pub const Y:    A3<FT> = A3(FT::ZERO, FT::ONE,  FT::ZERO);
    pub const Z:    A3<FT> = A3(FT::ZERO, FT::ZERO, FT::ONE );

    #[inline(always)]
    pub fn cw_inv(self) -> A3<FT> {
        self.map(FT::inv)
    }
}

#[inline(always)]
pub fn dot<A, B, Z>(a: A3<A>, b: A3<B>) -> Z
        where A: Mul<B, Output=Z>,
              Z: Add<Z, Output=Z> {
    zip(a, b, Mul::mul).reduce(Add::add)
}

impl<FT> Add for A3<FT> where FT: Float {
    type Output = A3<FT>;
    #[inline(always)]
    fn add(self, v: A3<FT>) -> A3<FT> {
        zip(self, v, Add::add)
    }
}

impl<FT> AddAssign for A3<FT> where FT: Float {
    #[inline(always)]
    fn add_assign(&mut self, v: A3<FT>) {
        self.0 += v.0;
        self.1 += v.1;
        self.2 += v.2;
    }
}

impl<FT> Sub for A3<FT> where FT: Float {
    type Output = A3<FT>;
    #[inline(always)]
    fn sub(self, v: A3<FT>) -> A3<FT> {
        zip(self, v, Sub::sub)
    }
}

impl<FT> Neg for A3<FT> where FT: Float {
    type Output = A3<FT>;
    #[inline(always)]
    fn neg(self) -> A3<FT> {
        self.map(Neg::neg)
    }
}

impl<FT> Mul<FT> for A3<FT> where FT: Float {
    type Output = A3<FT>;
    #[inline(always)]
    fn mul(self, f: FT) -> A3<FT> {
        zip(self, rep(f), Mul::mul)
    }
}

impl Mul<A3<f64>> for f64 {
    type Output = A3<f64>;
    #[inline(always)]
    fn mul(self, v: A3<f64>) -> A3<f64> {
        v * self
    }
}

impl Mul<A3<f32>> for f32 {
    type Output = A3<f32>;
    #[inline(always)]
    fn mul(self, v: A3<f32>) -> A3<f32> {
        v * self
    }
}

impl<FT> Div<FT> for A3<FT> where FT: Float {
    type Output = A3<FT>;
    #[inline(always)]
    fn div(self, f: FT) -> A3<FT> {
        self * f.inv()
    }
}

impl DivAssign<F> for F3 {
    #[inline(always)]
    fn div_assign(&mut self, f: F) {
        self.0 /= f;
        self.1 /= f;
        self.2 /= f;
    }
}
