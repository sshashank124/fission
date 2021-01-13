#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

use crate::color::Color;
use crate::shape::intersection::Its;
use crate::texture::Tex;
use crate::util::pdf::PDF;

#[derive(Debug, Deserialize)]
pub struct Infinite {
    intensity: Tex<Color>,
}

impl Infinite {
    #[inline] pub fn sample(&self, its: &Its, s: F2) -> (PDF<Color>, R) {
        let theta_phi = s * A2(F::PI, F::TWO_PI);
        let sray = R::unbounded(its.p, V::from(Frame::spher2cart(theta_phi)));
        (PDF::new(self.intensity.eval(s), Self::pdf(its, &sray)), sray)
    }

    #[inline] pub fn pdf(its: &Its, sray: &R) -> F
    { F::INV_2PI * F::INV_PI / F::sqrt(1. - F3::dot(its.n, sray.d).sq()) }

    #[inline] pub fn eval_env(&self, ray: &R) -> Color {
        let uv = Frame::cart2spher(F3::from(ray.d).swizzle(0, 2, 1));
        self.intensity.eval(uv * A2(F::INV_PI, F::INV_2PI))
    }

    #[inline] pub fn power(&self) -> F {
        // TODO how to incorporate scene bsphere surface area?
        self.intensity.mean().luminance() * F::PI * 100.
    }
}
