#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

use crate::color::Color;
use crate::texture::Tex;
use crate::util::pdf::Pdf;

#[derive(Debug, Default, Deserialize)]
pub struct Diffuse {
    albedo: Tex<Color>,
}

impl Diffuse {
    #[inline] pub fn eval(&self, wi: V, wo: V, uv: F2) -> Color {
        let cto = Frame::ct(wo);
        if Frame::ct(wi) <= 0. || cto <= 0. { Color::ZERO }
        else { self.albedo.eval(uv) * F::INV_PI * cto }
    }

    #[inline] pub fn sample(&self, uv: F2, s: F2) -> (Pdf<Color>, V, bool) {
        let wo = CosineHemisphere::warp(s);
        (Pdf::new(self.albedo.eval(uv), Self::pdf(wo)), wo.conv(), false)
    }

    #[inline] pub fn pdf(wo: impl Conv<F3>) -> F { CosineHemisphere::pdf(wo) }
}

impl Zero for Diffuse { const ZERO: Self = Self { albedo: Tex::ZERO }; }
