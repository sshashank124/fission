use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub,
               SubAssign};

#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::{Deserialize, Serialize};

pub type Color = RGB;

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize)]
#[repr(C)]
pub struct RGB(pub F3);

impl Zero for RGB { const ZERO: Self = Self(F3::ZERO); }
impl One for RGB { const ONE: Self = Self(F3::ONE); }

impl RGB {
    #[inline] pub const fn from_rgb(rgb: F3) -> Self { Self(rgb) }
    #[inline] pub const fn gray(g: F) -> Self { Self(F3::rep(g)) }
    #[inline] pub const fn to_rgb(self) -> Self { self }
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
