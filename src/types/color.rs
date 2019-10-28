use std::ops::{AddAssign, Div};

use super::*;


#[derive(Clone)]
pub struct Color(A3<f32>);

impl Color {
    #[inline]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color(A3(r, g, b))
    }

    pub const BLACK: Color = Color::rgb(0., 0., 0.);
    pub const WHITE: Color = Color::rgb(1., 1., 1.);
}

impl AddAssign for Color {
    #[inline]
    fn add_assign(&mut self, color: Color) {
        self.0 += color.0;
    }
}

impl Div<f32> for Color {
    type Output = Color;
    #[inline]
    fn div(self, f: f32) -> Color {
        Color(self.0 / f)
    }
}
