use super::*;

pub struct Dielectric {
    eta: F,
}

impl Dielectric {
    pub fn new(ior: Option<F2>) -> Self { Self { eta: eta(ior) } }
}

impl BXDF for Dielectric {
    #[inline(always)]
    fn eval(&self, _: V, _: V, _: F2) -> Color { Color::ZERO }

    #[inline(always)]
    fn sample(&self, wi: V, _: F2, s: F2) -> (Color, V, F, bool) {
        let (fr, ctt, eta) = fresnel(Frame::ct(*wi), self.eta);
        let (wo, p) = if s[0] <= fr {
            (Frame::reflect(wi), fr)
        } else {
            (V::v(-eta * wi[X], -eta * wi[Y], ctt).unit(), 1. - fr)
        };
        (Color::ONE, wo, p, self.is_delta())
    }

    #[inline(always)]
    fn pdf(&self, _: V, _: V) -> F { 0. }

    #[inline(always)]
    fn is_delta(&self) -> bool { true }
}
