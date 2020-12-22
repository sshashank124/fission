use super::*;

#[derive(Debug)]
pub struct Dielectric {
    eta: F,
}

impl Dielectric {
    pub fn new(ior: Option<F2>) -> Self { Self { eta: eta(ior) } }

    #[inline(always)]
    pub fn sample(&self, wi: V, s: F2) -> (Color, V, F, bool) {
        let (fr, ctt, eta) = fresnel(Frame::ct(wi), self.eta);
        let (wo, p) = if s[0] <= fr { (V::from(Frame::reflect(wi)), fr) } else {
            (V::from(A3(-eta * wi[X], -eta * wi[Y], ctt)).unit(), 1. - fr)
        };
        (Color::ONE, wo, p, self.is_delta())
    }

    #[inline(always)] pub fn is_delta(&self) -> bool { true }
}
