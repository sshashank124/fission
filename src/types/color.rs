use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul, Div, DivAssign, Deref};

use super::*;
use crate::op;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color(A3<F>);

impl Color {
    #[inline(always)]
    pub const fn rgb(r: F, g: F, b: F) -> Color { Color(A3(r, g, b)) }

    pub const BLACK: Color = Color(F3::ZERO);
    pub const WHITE: Color = Color(F3::ONE);
}

op!(Add::add, *Color -> *Color -> Color);
op!(Mul::mul, *Color ->      F -> Color);
op!(Div::div, *Color ->      F -> Color);
op!(AddAssign::add_assign, *mut Color -> *Color -> ());
op!(DivAssign::div_assign, *mut Color ->      F -> ());

impl Sum<Color> for Color {
    #[inline(always)]
    fn sum<I: Iterator<Item=Color>>(iter: I) -> Color {
        iter.fold(Color::BLACK, Add::add)
    }
}

impl Deref for Color {
    type Target = F3;
    #[inline(always)] fn deref(&self) -> &Self::Target { &self.0 }
}
