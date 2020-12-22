use super::*;

use crate::sampler::*;

#[derive(Debug)]
pub struct Microfacet {
    kd:    Color,
    ks:    F,
    alpha: F,
    eta:   F,
}

impl Microfacet {
    pub fn new(kd: Color, alpha: Option<F>, ior: Option<F2>) -> Self {
        let ks = 1. - kd.max_channel();
        let alpha = alpha.unwrap_or(0.1);
        let eta = eta(ior);
        Self { kd, ks, alpha, eta }
    }

    #[inline(always)] fn beckmann(&self, v: V) -> F {
        F::exp(-Frame::t2t(v) / self.alpha.sq()) * F::INV_PI
        / (self.alpha * Frame::c2t(v)).sq()
    }

    #[inline(always)] fn smith_beckmann_g1(&self, v: V, n: V) -> F {
        let tt = Frame::tt(v);
        if tt == 0. { return 1. }
        if F3::dot(n, v) * Frame::ct(v) <= 0. { return 0. }
        let a = (self.alpha * tt).inv();
        if a >= 1.6 { return 1. }
        (3.535 * a + 2.181 * a.sq()) / (1. + 2.276 * a + 2.577 * a.sq())
    }

    #[inline(always)] pub fn eval(&self, wi: V, wo: V) -> Color {
        let cti = Frame::ct(wi);
        let cto = Frame::ct(wo);
        if cti <= 0. || cto <= 0. {
            return Color::ZERO
        }
        let wh = wi + wo;
        let beck = self.beckmann(wh);
        let (fr, _, _) = fresnel(F3::dot(wh, wi), self.eta);
        let g = self.smith_beckmann_g1(wi, wh) * self.smith_beckmann_g1(wo, wh);
        self.kd * F::INV_PI * cto + (self.ks * beck * fr * g * 0.25) / cti
    }

    #[inline(always)]
    pub fn sample(&self, wi: V, s: F2) -> (Color, V, F, bool) {
        let spec = |s| {
            let n = V::from(BeckmannHemisphere::warp(s, self.alpha));
            (n * 2. * F3::dot(n, wi) - wi).unit()
        };
        let diffuse = |s| V::from(CosineHemisphere::warp(s));
        let wo = Sampler::split_reuse_2d(s, self.ks, spec, diffuse);
        let p = self.pdf(wi, wo);
        (if p <= 0. { Color::ZERO } else { self.eval(wi, wo) / p },
         wo, p, self.is_delta())
    }

    #[inline(always)] pub fn pdf(&self, wi: V, wo: V) -> F {
        let wh = wi + wo;
        let dp = CosineHemisphere::pdf(wo);
        let sp = self.beckmann(wh) * Frame::ct(wh) * 0.25 / F3::dot(wh, wo);
        LinearScale::interp(A2(dp, sp), self.ks)
    }

    #[inline(always)] pub fn is_delta(&self) -> bool { false }
}
