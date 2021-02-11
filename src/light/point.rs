#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

use crate::color::Color;
use crate::shape::intersection::Its;
use crate::util::pdf::Pdf;

#[derive(Debug, Deserialize)]
pub struct Point {
    #[serde(rename="power", deserialize_with="de_intensity")]
    intensity: Color,
    position:  P,
}

impl Point {
    #[inline] pub fn sample(&self, its: &Its) -> (Pdf<Color>, R) {
        let sray = R::p2(its.p, self.position);
        (Pdf::sole(self.intensity / sray.t.sq()), sray)
    }

    #[inline] pub fn power(&self) -> F { self.intensity.luminance() * F::FOUR_PI }
}

fn de_intensity<'de, D>(de: D) -> Result<Color, D::Error>
where D: serde::Deserializer<'de>
{ Color::deserialize(de).map(|power| power * F::INV_4PI) }
