use super::*;


pub struct Diffuse {
    albedo: Tex<Color>,
}

impl Diffuse
{ pub const fn new(albedo: Tex<Color>) -> Self { Self { albedo } } }

impl BXDF for Diffuse {
    #[inline(always)] fn eval(&self, wi: V, wo: V, uv: F2) -> Color {
        let cto = Frame::ct(*wo);
        if Frame::ct(*wi) <= 0. || cto <= 0. { Color::BLACK }
        else { self.albedo.eval(uv) * F::INV_PI * cto }
    }

    #[inline(always)]
    fn sample(&self, wi: V, uv: F2, s: F2) -> (Color, V, F, bool)
    { let wo = V(CosineHemisphere::warp(s, ()));
      (self.albedo.eval(uv), wo, self.pdf(wi, wo), self.is_delta()) }

    #[inline(always)] fn pdf(&self, _: V, wo: V) -> F
    { CosineHemisphere::pdf(*wo, ()) }

    #[inline(always)] fn is_delta(&self) -> bool { false }
}

impl Zero for Diffuse { const ZERO: Self = Self::new(Tex::ZERO); }
