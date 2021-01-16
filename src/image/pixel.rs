use std::ops::AddAssign;

#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::{Deserialize, Serialize};

use crate::color::Color;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Pixel {
    val: Color,
    w:   F,
}

impl Pixel {
    #[inline] pub fn to_color(&self) -> Color
    { self.val / if F::approx_zero(self.w) { 1. } else { self.w } }

    #[inline] pub fn to_rgbw(&self) -> [f32; 4] {
        let rgb = self.val.to_rgb().0.conv();
        [rgb.0, rgb.1, rgb.2, f32::of(self.w)]
    }
}

impl Zero for Pixel
{ const ZERO: Self = Self { val: Color::ZERO, w: F::ZERO }; }

impl AddAssign for Pixel {
    #[inline] fn add_assign(&mut self, pixel: Self) {
        self.val += pixel.val;
        self.w += pixel.w;
    }
}

impl AddAssign<Color> for Pixel {
    #[inline] fn add_assign(&mut self, color: Color) { *self += Pixel { val: color, w: 1. }; }
}
