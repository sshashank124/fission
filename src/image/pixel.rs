use std::ops::AddAssign;

use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Pixel {
    val: Color,
    w:   F,
}

impl Pixel {
    #[inline] pub fn eval(&self) -> Color
    { self.val / if F::approx_zero(self.w) { 1. } else { self.w } }
}

impl Zero for Pixel
{ const ZERO: Self = Self { val: Color::ZERO, w: F::ZERO }; }

impl AddAssign<Color> for Pixel {
    #[inline] fn add_assign(&mut self, color: Color) {
        self.val += color;
        self.w += 1.;
    }
}

impl AddAssign for Pixel {
    #[inline] fn add_assign(&mut self, pixel: Self) {
        self.val += pixel.val;
        self.w += pixel.w;
    }
}
