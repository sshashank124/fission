use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub,
               SubAssign};

#[allow(clippy::wildcard_imports)]
use graphite::*;
use image::Rgba;
use serde::{Deserialize, Serialize};

pub type Color = Rgb;

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize)]
#[repr(C)]
pub struct Rgb(pub F3);

impl Zero for Rgb { const ZERO: Self = Self(F3::ZERO); }
impl One for Rgb { const ONE: Self = Self(F3::ONE); }

impl Rgb {
    #[inline] pub fn max_channel(self) -> F { self.0.max() }

    const SENSITIVITIES: F3 = A3(0.212671, 0.715160, 0.072169);
    #[inline] pub fn luminance(self) -> F { F3::dot(self.0, Self::SENSITIVITIES) }
}

op!(Neg::neg, *Rgb);
op!(Inv::inv, *Rgb);
op!(Add::add, *Rgb -> *Rgb -> Rgb);
op!(Sub::sub, *Rgb -> *Rgb -> Rgb);
op!(Mul::mul, *Rgb -> *Rgb -> Rgb);
op!(Div::div, *Rgb -> *Rgb -> Rgb);
op!(AddAssign::add_assign, *mut Rgb -> *Rgb -> ());
op!(SubAssign::sub_assign, *mut Rgb -> *Rgb -> ());
op!(MulAssign::mul_assign, *mut Rgb -> *Rgb -> ());
op!(DivAssign::div_assign, *mut Rgb -> *Rgb -> ());
op!(Add::add, *Rgb -> F -> Rgb);
op!(Sub::sub, *Rgb -> F -> Rgb);
op!(Mul::mul, *Rgb -> F -> Rgb);
op!(Div::div, *Rgb -> F -> Rgb);
op!(AddAssign::add_assign, *mut Rgb -> F -> ());
op!(SubAssign::sub_assign, *mut Rgb -> F -> ());
op!(MulAssign::mul_assign, *mut Rgb -> F -> ());
op!(DivAssign::div_assign, *mut Rgb -> F -> ());

impl Sum for Rgb {
    #[inline]
    fn sum<It>(it: It) -> Self where It: Iterator<Item=Self>
    { Self(it.map(|i| i.0).sum()) }
}

impl Product for Rgb {
    #[inline]
    fn product<It>(it: It) -> Self where It: Iterator<Item=Self>
    { Self(it.map(|i| i.0).product()) }
}

/* Convert Types to Rgb */
impl Conv<Rgb> for F3 { #[inline] fn conv(self) -> Rgb { Rgb(self) } }

impl Conv<Rgb> for F { #[inline] fn conv(self) -> Rgb { Rgb(F3::rep(self)) } }

impl Conv<Rgb> for A3<u8>
{ #[inline] fn conv(self) -> Rgb { Rgb(self.map(F::of).map(|f| f / 255.).map(gamma_correct_inv)) } }

impl Conv<Rgb> for Rgba<u8>
{ #[inline] fn conv(self) -> Rgb { Rgb::of(A3(self.0[0], self.0[1], self.0[2])) } }

/* Convert Rgb to Types */
impl Conv<F3> for Rgb { #[inline] fn conv(self) -> F3 { self.0 } }

impl Conv<Rgb> for Rgb { #[inline] fn conv(self) -> Self { self } }


#[inline] fn gamma_correct_inv(f: F) -> F
{ if f <= 0.04045 { f / 12.92 } else { ((f + 0.055) / 1.055).powf(2.4) } }
