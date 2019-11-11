use std::ops::{Add, Sub, Mul, Div, Neg};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};


pub trait Zero { const ZERO: Self; }
pub trait One { const ONE: Self; }

pub trait Num: Copy
             + Zero + One
             + Neg<Output=Self>
             + Add<Self, Output=Self> + AddAssign<Self>
             + Sub<Self, Output=Self> + SubAssign<Self>
             + Mul<Self, Output=Self> + MulAssign<Self>
             + Div<Self, Output=Self> + DivAssign<Self>
{
    fn abs(self) -> Self;
    fn sq(self) -> Self;

    fn min(a: Self, b: Self) -> Self;
    fn max(a: Self, b: Self) -> Self;
}

pub trait Inv {
    type Output;
    fn inv(self) -> Self;
}

pub trait Float: Num + Inv {
    const NEG_INF: Self;
    const POS_INF: Self;
    const EPSILON: Self;

    const PI: Self;

    fn sqrt(self) -> Self;

    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tand(self) -> Self;
}


impl Zero for f32 { const ZERO: Self = 0.; }
impl One for f32 { const ONE: Self = 1.; }

impl Num for f32 {
    #[inline(always)] fn abs(self) -> Self { self.abs() }
    #[inline(always)] fn sq(self) -> Self { self * self }

    #[inline(always)]
    fn min(a: Self, b: Self) -> Self { if a < b { a } else { b } }
    #[inline(always)]
    fn max(a: Self, b: Self) -> Self { if a > b { a } else { b } }
}

impl Inv for f32 {
    type Output = Self;
    #[inline(always)] fn inv(self) -> Self { self.recip() }
}

impl Float for f32 {
    const NEG_INF: Self = std::f32::NEG_INFINITY;
    const POS_INF: Self = std::f32::INFINITY;
    const EPSILON: Self = 1e-4;

    const PI: Self = std::f32::consts::PI;

    #[inline(always)] fn sqrt(self) -> Self { self.sqrt() }

    #[inline(always)] fn sin(self) -> Self { self.sin() }
    #[inline(always)] fn cos(self) -> Self { self.cos() }
    #[inline(always)] fn tand(self) -> Self { self.to_radians().tan() }
}


impl Zero for f64 { const ZERO: Self = 0.; }
impl One for f64 { const ONE: Self = 1.; }

impl Num for f64 {
    #[inline(always)] fn abs(self) -> Self { self.abs() }
    #[inline(always)] fn sq(self) -> Self { self * self }

    #[inline(always)]
    fn min(a: Self, b: Self) -> Self { if a < b { a } else { b } }
    #[inline(always)]
    fn max(a: Self, b: Self) -> Self { if a > b { a } else { b } }
}

impl Inv for f64 {
    type Output = Self;
    #[inline(always)] fn inv(self) -> Self { self.recip() }
}

impl Float for f64 {
    const NEG_INF: Self = std::f64::NEG_INFINITY;
    const POS_INF: Self = std::f64::INFINITY;
    const EPSILON: Self = 1e-7;

    const PI: Self = std::f64::consts::PI;

    #[inline(always)] fn sqrt(self) -> Self { self.sqrt() }

    #[inline(always)] fn sin(self) -> Self { self.sin() }
    #[inline(always)] fn cos(self) -> Self { self.cos() }
    #[inline(always)] fn tand(self) -> Self { self.to_radians().tan() }
}
