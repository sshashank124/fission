use super::*;


pub type PhaseFn = HenyeyGreenstein;

pub trait PhaseFunction {
    fn eval(&self, wi: V, wo: V) -> F;
    fn sample(&self, wo: V, s: F2) -> (F, V);
}

#[derive(Debug, PartialEq)]
pub struct HenyeyGreenstein {
    g: F,
}

impl HenyeyGreenstein {
    #[allow(dead_code)]
    pub fn new(g: F) -> Self { Self { g } }

    #[inline(always)] fn f(&self, ct: F) -> F {
        let denom = 1. + 2. * ct * self.g + self.g.sq();
        F::INV_4PI * (1. - self.g.sq()) / (denom * F::sqrt(denom))
    }
}

impl PhaseFunction for HenyeyGreenstein {
    #[inline(always)] fn eval(&self, wi: V, wo: V) -> F { self.f(wi.dot(wo)) }

    #[inline(always)] fn sample(&self, wo: V, s: F2) -> (F, V) {
        let ct = -if F::approx_zero(self.g) { F::ONE - s[0] * 2. as F } else {
            let t: F = (1. - self.g.sq()) / (1. - self.g + 2. * self.g * s[0]);
            (1. + self.g.sq() - t.sq()) / (2. * self.g)
        };
        let st = F::sqrt(F::clamp_pos(1. - ct.sq()));
        let phi = F::TWO_PI * s[1];
        (self.f(ct), T::from_frame(wo) * V(Frame::trig2cart(ct, st, phi)))
    }
}
