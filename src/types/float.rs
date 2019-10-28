use std::ops::{Add, AddAssign, Sub, Mul, Div, Neg};


pub trait Float: Copy
               + Add<Self, Output=Self>
               + Sub<Self, Output=Self>
               + Mul<Self, Output=Self>
               + Div<Self, Output=Self>
               + AddAssign<Self>
               + Neg<Output=Self> {

    const NEG_INF: Self;
    const POS_INF: Self;
    const ZERO: Self;
    const ONE: Self;
    const EPSILON: Self;

    fn inv(self) -> Self;
    fn tand(self) -> Self;
}

impl Float for f32 {
    const NEG_INF: f32 = std::f32::NEG_INFINITY;
    const POS_INF: f32 = std::f32::INFINITY;
    const ZERO: f32 = 0.;
    const ONE: f32 = 1.;
    const EPSILON: f32 = 1e-7;

    #[inline]
    fn inv(self) -> f32 {
        self.recip()
    }

    #[inline]
    fn tand(self) -> f32 {
        self.to_radians().tan()
    }
}

impl Float for f64 {
    const NEG_INF: f64 = std::f64::NEG_INFINITY;
    const POS_INF: f64 = std::f64::INFINITY;
    const ZERO: f64 = 0.;
    const ONE: f64 = 1.;
    const EPSILON: f64 = 1e-7;

    #[inline]
    fn inv(self) -> f64 {
        self.recip()
    }

    #[inline]
    fn tand(self) -> f64 {
        self.to_radians().tan()
    }
}
