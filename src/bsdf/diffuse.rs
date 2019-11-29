use super::*;


pub struct Diffuse {
    albedo: Tex<Color>,
}

impl Diffuse {
    #[inline(always)] pub const fn new(albedo: Tex<Color>) -> Self
    { Self { albedo } }
}

impl Bxdf for Diffuse {
    #[inline(always)] fn eval(&self, (wi, wo, uv): BsdfQuery) -> Color {
        if F::is_nonpos(wi[Z]) || F::is_nonpos(wo[Z]) { Color::BLACK }
        else { self.albedo.eval(uv) * F::INV_PI }
    }
}

impl Zero for Diffuse { const ZERO: Self = Self::new(Tex::ZERO); }
