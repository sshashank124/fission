use std::iter::Sum;
use std::ops::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Neg};

use super::*;
use crate::op;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color(pub F3);

impl Color {
    #[inline(always)]
    pub fn gray(g: F) -> Self { Self(A3::rep(g)) }

    pub const BLACK: Color = Self::ZERO;
    pub const WHITE: Color = Self::ONE;
}

impl Zero for Color {
    const ZERO: Self = Self(A3::ZERO);
}
impl One for Color {
    const ONE: Self = Self(A3::ONE);
}

op!(Neg::neg, *Color);
op!(Add::add, *Color -> *Color -> Color);
op!(Add::add, *Color ->      F -> Color);
op!(Mul::mul, *Color -> *Color -> Color);
op!(Mul::mul, *Color ->      F -> Color);
op!(Div::div, *Color ->      F -> Color);
op!(AddAssign::add_assign, *mut Color -> *Color -> ());
op!(MulAssign::mul_assign, *mut Color -> *Color -> ());
op!(DivAssign::div_assign, *mut Color ->      F -> ());

impl Sum<Color> for Color {
    #[inline(always)]
    fn sum<I: Iterator<Item = Color>>(iter: I) -> Color {
        iter.fold(Color::BLACK, Add::add)
    }
}

impl Deref for Color {
    type Target = F3;
    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.0 }
}
