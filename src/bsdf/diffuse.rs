use super::*;


pub struct Diffuse {
    albedo: Tex<Color>,
}

impl Diffuse {
    #[inline(always)] pub const fn new(albedo: Tex<Color>) -> Self
    { Self { albedo } }
}

impl Bxdf for Diffuse {
    #[inline(always)] fn eval(&self, wi: V, wo: V, uv: F2) -> Color
    { if F::is_nonpos(wi[Z]) || F::is_nonpos(wo[Z]) { Color::BLACK }
      else { self.albedo.eval(uv) * F::INV_PI } }

    #[inline(always)] fn sample(&self, wo: V, uv: F2, s: F2) -> (Color, V)
    { let wi = V(CosineHemisphere::warp(s));
      (self.eval(wi, wo, uv), wi) }
}

impl Zero for Diffuse { const ZERO: Self = Self::new(Tex::ZERO); }
