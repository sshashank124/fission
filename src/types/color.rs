use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign};

use super::*;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color(A3<f32>);

impl Color {
    #[inline(always)]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color(A3(r, g, b))
    }

    pub const BLACK: Color = Color::rgb(0., 0., 0.);
    pub const WHITE: Color = Color::rgb(1., 1., 1.);
}

impl Add for Color {
    type Output = Color;
    #[inline(always)]
    fn add(self, color: Color) -> Color {
        Color(self.0 + color.0)
    }
}

impl AddAssign for Color {
    #[inline(always)]
    fn add_assign(&mut self, color: Color) {
        self.0 += color.0;
    }
}

impl Div<F> for Color {
    type Output = Color;
    #[inline(always)]
    fn div(self, f: F) -> Color {
        Color(self.0 / f)
    }
}

impl Div<I> for Color {
    type Output = Color;
    #[inline(always)]
    fn div(self, i: I) -> Color {
        self / i as F
    }
}

impl DivAssign<F> for Color {
    #[inline(always)]
    fn div_assign(&mut self, f: F) {
        self.0 /= f;
    }
}

impl DivAssign<I> for Color {
    #[inline(always)]
    fn div_assign(&mut self, i: I) {
        self.0 /= i as F;
    }
}

impl Sum<Color> for Color {
    #[inline(always)]
    fn sum<I: Iterator<Item=Color>>(iter: I) -> Color {
        iter.fold(Color::BLACK, Add::add)
    }
}
