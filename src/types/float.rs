use std::ops::{Add, AddAssign, Sub, Mul, Div, DivAssign, Neg};


pub trait Float: Copy
               + Add<Self, Output=Self>
               + Sub<Self, Output=Self>
               + Mul<Self, Output=Self>
               + Div<Self, Output=Self>
               + DivAssign<Self>
               + AddAssign<Self>
               + Neg<Output=Self> {

    const NEG_INF: Self;
    const POS_INF: Self;
    const ZERO: Self;
    const ONE: Self;
    const EPSILON: Self;

    const PI: Self;

    fn abs(self) -> Self;
    fn sq(self) -> Self;
    fn sqrt(self) -> Self;
    fn inv(self) -> Self;

    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tand(self) -> Self;

    fn min(a: Self, b: Self) -> Self;
    fn max(a: Self, b: Self) -> Self;
}

impl Float for f32 {
    const NEG_INF: f32 = std::f32::NEG_INFINITY;
    const POS_INF: f32 = std::f32::INFINITY;
    const ZERO: f32 = 0.;
    const ONE: f32 = 1.;
    const EPSILON: f32 = 2e-4;

    const PI: f32 = std::f32::consts::PI;

    #[inline(always)]
    fn abs(self) -> f32 {
        self.abs()
    }

    #[inline(always)]
    fn sq(self) -> f32 {
        self * self
    }

    #[inline(always)]
    fn sqrt(self) -> f32 {
        self.sqrt()
    }

    #[inline(always)]
    fn inv(self) -> f32 {
        self.recip()
    }

    #[inline(always)]
    fn sin(self) -> f32 {
        self.sin()
    }

    #[inline(always)]
    fn cos(self) -> f32 {
        self.cos()
    }

    #[inline(always)]
    fn tand(self) -> f32 {
        self.to_radians().tan()
    }

    #[inline(always)]
    fn min(a: f32, b: f32) -> f32 {
        if a < b { a } else { b }
    }

    #[inline(always)]
    fn max(a: f32, b: f32) -> f32 {
        if a > b { a } else { b }
    }
}

impl Float for f64 {
    const NEG_INF: f64 = std::f64::NEG_INFINITY;
    const POS_INF: f64 = std::f64::INFINITY;
    const ZERO: f64 = 0.;
    const ONE: f64 = 1.;
    const EPSILON: f64 = 1e-7;

    const PI: f64 = std::f64::consts::PI;

    #[inline(always)]
    fn abs(self) -> f64 {
        self.abs()
    }

    #[inline(always)]
    fn sq(self) -> f64 {
        self * self
    }

    #[inline(always)]
    fn sqrt(self) -> f64 {
        self.sqrt()
    }

    #[inline(always)]
    fn inv(self) -> f64 {
        self.recip()
    }

    #[inline(always)]
    fn sin(self) -> f64 {
        self.sin()
    }

    #[inline(always)]
    fn cos(self) -> f64 {
        self.cos()
    }

    #[inline(always)]
    fn tand(self) -> f64 {
        self.to_radians().tan()
    }

    #[inline(always)]
    fn min(a: f64, b: f64) -> f64 {
        if a < b { a } else { b }
    }

    #[inline(always)]
    fn max(a: f64, b: f64) -> f64 {
        if a > b { a } else { b }
    }
}
