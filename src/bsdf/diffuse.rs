use super::*;

#[derive(Debug, Default, Deserialize)]
pub struct Diffuse {
    albedo: Tex<Color>,
}

impl Diffuse {
    #[inline(always)] pub fn eval(&self, wi: V, wo: V, uv: F2) -> Color {
        let cto = Frame::ct(wo);
        if Frame::ct(wi) <= 0. || cto <= 0. { Color::ZERO }
        else { self.albedo.eval(uv) * F::INV_PI * cto }
    }

    #[inline(always)]
    pub fn sample(&self, uv: F2, s: F2) -> (Color, V, F, bool) {
        let wo = V::from(CosineHemisphere::warp(s));
        (self.albedo.eval(uv), wo, self.pdf(wo), self.is_delta())
    }

    #[inline(always)] pub fn pdf(&self, wo: V) -> F
    { CosineHemisphere::pdf(wo) }

    #[inline(always)] pub fn is_delta(&self) -> bool { false }
}

impl Zero for Diffuse { const ZERO: Self = Self { albedo: Tex::ZERO }; }
