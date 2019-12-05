use super::*;


pub struct Dielectric {
    ior: F2,
}

impl Dielectric {
    #[inline(always)] pub fn new(ior: Option<F2>) -> Self
    { Self { ior: ior.unwrap_or(A2(1.5046, 1.000_277)) } }
}

impl Bxdf for Dielectric {
    #[inline(always)] fn eval(&self, _: V, _: V, _: F2) -> Color
    { Color::ZERO }

    #[inline(always)] fn sample(&self, wi: V, _: F2, s: F2) -> (Color, V, F) {
        let cti = Frame::ct(*wi);
        let fr = fresnel(cti, self.ior.rev());
        let wo = if s[0] < fr { Frame::reflect(wi) } else {
            let (eta, o) = if cti < 0. { (self.ior[0] / self.ior[1], -1.) }
                           else { (self.ior[1] / self.ior[0], 1.) };
            V::Z * o * F::sqrt(1. - F::sq(eta) * (1. - cti.sq()))
                - (wi - V::Z * cti) * -eta
        };
        (Color::ONE, wo, 1.)
    }

    #[inline(always)] fn pdf(&self, _: V, _: V) -> F { 0. }
}
