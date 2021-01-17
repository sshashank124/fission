use std::ops::AddAssign;

#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::{Deserialize, Serialize};

use crate::color::{Color, RGB};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Pixel {
    val: Color,
    w:   F,
}

impl Zero for Pixel
{ const ZERO: Self = Self { val: Color::ZERO, w: F::ZERO }; }

impl AddAssign for Pixel {
    #[inline] fn add_assign(&mut self, pixel: Self) {
        self.val += pixel.val;
        self.w += pixel.w;
    }
}

impl AddAssign<Color> for Pixel
{ #[inline] fn add_assign(&mut self, color: Color) { *self += Self { val: color, w: 1. }; } }

impl Conv<Color> for Pixel
{ #[inline] fn conv(self) -> RGB { if self.w == 0. { Color::ZERO } else { self.val / self.w } } }

impl Conv<[f32; 4]> for Pixel {
    #[inline] fn conv(self) -> [f32; 4] {
        let rgb = conv!(self.val => RGB => F3 => A3<f32>);
        [rgb.0, rgb.1, rgb.2, conv!(self.w => f32)]
    }
}
