use std::ops::{Add, Sub, Mul, Div, Neg};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

use super::*;


pub trait Zero: Copy { const ZERO: Self; }
pub trait One: Copy { const ONE: Self; }

pub trait Num: Copy + PartialOrd
             + Zero + One
             + Neg<Output=Self>
             + Add<Self, Output=Self> + AddAssign<Self>
             + Sub<Self, Output=Self> + SubAssign<Self>
             + Mul<Self, Output=Self> + MulAssign<Self>
             + Div<Self, Output=Self> + DivAssign<Self>
{
    #[inline(always)] fn sq(self) -> Self { self * self }

    #[inline(always)]
    fn abs(a: Self) -> Self { if a >= Self::ZERO { a } else { -a } }

    #[inline(always)]
    fn min(a: Self, b: Self) -> Self { if a < b { a } else { b } }

    #[inline(always)]
    fn max(a: Self, b: Self) -> Self { if a < b { b } else { a } }

    #[inline(always)]
    fn clamp(v: Self, a: Self, b: Self) -> Self { Num::min(Num::max(v, a), b) }

    #[inline(always)]
    fn clamp_pos(v: Self) -> Self { Num::max(v, Self::ZERO) }

    #[inline(always)]
    fn clamp_unit(v: Self) -> Self { Num::clamp(v, Self::ZERO, Self::ONE) }
}

impl Zero for I { const ZERO: Self = 0; }
impl One  for I { const ONE: Self = 1; }
impl Num  for I { }

impl Zero for F { const ZERO: Self = 0.; }
impl One  for F { const ONE: Self = 1.; }
impl Num  for F { }


pub trait Inv {
    type Output;
    fn inv(self) -> Self;
}

pub trait Float: Num + Inv {
    const NEG_INF: Self;
    const POS_INF: Self;
    const EPSILON: Self;

    const PI: Self;

    const FRAC_1_2POW32: Self;

    fn ceili(self) -> I;
    fn floori(self) -> I;

    fn exp(f: Self) -> Self;
    fn sqrt(self) -> Self;

    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn sind(self) -> Self;
    fn cosd(self) -> Self;
    fn tand(self) -> Self;

    #[inline(always)]
    fn approx_eq(a: Self, b: Self) -> bool { Self::abs(a - b) < Self::EPSILON }
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

    const FRAC_1_2POW32: Self = 2.328_306_4e-10;

    #[inline(always)] fn ceili(self) -> I { self.ceil() as I }
    #[inline(always)] fn floori(self) -> I { self.floor() as I }

    #[inline(always)] fn exp(f: Self) -> Self { f.exp() }
    #[inline(always)] fn sqrt(self) -> Self { self.sqrt() }

    #[inline(always)] fn sin(self) -> Self { self.sin() }
    #[inline(always)] fn cos(self) -> Self { self.cos() }
    #[inline(always)] fn tan(self) -> Self { self.tan() }
    #[inline(always)] fn sind(self) -> Self { self.to_radians().sin() }
    #[inline(always)] fn cosd(self) -> Self { self.to_radians().cos() }
    #[inline(always)] fn tand(self) -> Self { self.to_radians().tan() }
}


#[inline(always)]
pub fn ceil_pow2_u32(i: u32) -> u32 {
    1 << (32 - i.saturating_sub(1).leading_zeros())
}

#[inline(always)]
pub fn log2_ceil_u32(i: u32) -> u32 { 31 - i.leading_zeros() }
