use super::*;


pub struct Diffuse {
    albedo: Color,
}

impl Diffuse {
    #[inline(always)] pub const fn new(albedo: Color) -> Self
    { Self { albedo } }
}

impl BXDF for Diffuse {
    #[inline(always)] fn eval(&self, wi: V, wo: V) -> Color {
        if F::is_nonpos(wi[Z]) || F::is_nonpos(wo[Z]) { Color::BLACK }
        else { self.albedo * F::INV_PI }
    }
}

impl One for Diffuse { const ONE: Self = Self::new(Color::WHITE); }
