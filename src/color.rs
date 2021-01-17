use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub,
               SubAssign};

#[allow(clippy::wildcard_imports)]
use graphite::*;
use image::Rgba;
use serde::{Deserialize, Serialize};

pub type Color = RGB;

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize)]
#[repr(C)]
pub struct RGB(pub F3);

impl Zero for RGB { const ZERO: Self = Self(F3::ZERO); }
impl One for RGB { const ONE: Self = Self(F3::ONE); }

impl RGB {
    #[inline] pub fn max_channel(self) -> F { self.0.max() }

    const SENSITIVITIES: F3 = A3(0.212671, 0.715160, 0.072169);
    #[inline] pub fn luminance(self) -> F { F3::dot(self.0, Self::SENSITIVITIES) }
}

op!(Neg::neg, *RGB);
op!(Inv::inv, *RGB);
op!(Add::add, *RGB -> *RGB -> RGB);
op!(Sub::sub, *RGB -> *RGB -> RGB);
op!(Mul::mul, *RGB -> *RGB -> RGB);
op!(Div::div, *RGB -> *RGB -> RGB);
op!(AddAssign::add_assign, *mut RGB -> *RGB -> ());
op!(SubAssign::sub_assign, *mut RGB -> *RGB -> ());
op!(MulAssign::mul_assign, *mut RGB -> *RGB -> ());
op!(DivAssign::div_assign, *mut RGB -> *RGB -> ());
op!(Add::add, *RGB -> F -> RGB);
op!(Sub::sub, *RGB -> F -> RGB);
op!(Mul::mul, *RGB -> F -> RGB);
op!(Div::div, *RGB -> F -> RGB);
op!(AddAssign::add_assign, *mut RGB -> F -> ());
op!(SubAssign::sub_assign, *mut RGB -> F -> ());
op!(MulAssign::mul_assign, *mut RGB -> F -> ());
op!(DivAssign::div_assign, *mut RGB -> F -> ());

impl Sum for RGB {
    #[inline]
    fn sum<It>(it: It) -> Self where It: Iterator<Item=Self>
    { Self(it.map(|i| i.0).sum()) }
}

impl Product for RGB {
    #[inline]
    fn product<It>(it: It) -> Self where It: Iterator<Item=Self>
    { Self(it.map(|i| i.0).product()) }
}

/* Convert Types to RGB */
impl Conv<RGB> for F3 { #[inline] fn conv(self) -> RGB { RGB(self) } }

impl Conv<RGB> for F { #[inline] fn conv(self) -> RGB { RGB(F3::rep(self)) } }

impl Conv<RGB> for A3<u8>
{ #[inline] fn conv(self) -> RGB { RGB(self.map(F::of).map(|f| f / 255.).map(gamma_correct_inv)) } }

impl Conv<RGB> for Rgba<u8>
{ #[inline] fn conv(self) -> RGB { RGB::of(A3(self.0[0], self.0[1], self.0[2])) } }

/* Convert RGB to Types */
impl Conv<F3> for RGB { #[inline] fn conv(self) -> F3 { self.0 } }

impl Conv<RGB> for RGB { #[inline] fn conv(self) -> Self { self } }


#[inline] fn gamma_correct_inv(f: F) -> F
{ if f <= 0.04045 { f / 12.92 } else { ((f + 0.055) / 1.055).powf(2.4) } }
