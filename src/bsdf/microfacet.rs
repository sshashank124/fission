use std::ops::Sub;

use super::*;

use crate::sampler::*;


pub struct Microfacet {
    kd: Color,
    ks: F,
    alpha: F,
    ior: F2,
}

impl Microfacet {
    #[inline(always)]
    pub fn new(kd: Color, alpha: Option<F>, ior: Option<F2>) -> Self {
        let ks = 1. - kd.reduce(F::max);
        let alpha = alpha.unwrap_or(0.1);
        let ior = ior.unwrap_or(A2(1.5046, 1.000277));
        Self { kd, ks, alpha, ior }
    }

    #[inline(always)] fn beckmann(&self, n: N) -> F
    { F::exp(-Frame::t2t(**n) / self.alpha.sq()) * F::INV_PI
          / (self.alpha * Frame::c2t(**n)).sq() }

    #[inline(always)] fn smith_beckmann_g1(&self, v: V, n: N) -> F {
        let tt = Frame::tt(*v);
        if tt == 0. { return 1. }
        if n.dot(v) * Frame::ct(*v) <= 0. { return 0. }
        let a = (self.alpha * tt).inv();
        if a >= 1.6 { return 1. }
        (3.535 * a + 2.181 * a.sq()) / (1. + 2.276 * a + 2.577 * a.sq())
    }

    #[inline(always)] fn fresnel(mut cti: F, mut eior: F2) -> F {
        if eior[0] == eior[1] { return 0. }
        if cti < 0. { eior = eior.rev(); cti = -cti; };
        let eta: F = eior[0] / eior[1];
        let stt2 = eta.sq() * (1. - cti.sq());
        if stt2 > 1. { return 1. }
        let ct = A2(cti, F::sqrt(1. - stt2));
        let iors = A2(eior, eior.rev());
        let d = iors.map(|ior| ior.dot(ct));
        let r = iors.map(|ior| (ior * ct).reduce(Sub::sub)) / d;
        r.map(F::sq).mean()
    }
}

impl Bxdf for Microfacet {
    #[inline(always)] fn eval(&self, wi: V, wo: V, _: F2) -> Color {
        let cti = Frame::ct(*wi); let cto = Frame::ct(*wo);
        if cti <= 0. || cto <= 0. { return Color::BLACK; }
        let wh = N::v(wi + wo);
        let beck = self.beckmann(wh);
        let fr = Self::fresnel(wh.dot(wi), self.ior.rev());
        let g = self.smith_beckmann_g1(wi, wh)
              * self.smith_beckmann_g1(wo, wh);
        self.kd * F::INV_PI + (self.ks * beck * fr * g * 0.25) / (cti * cto)
    }

    #[inline(always)] fn sample_dir(&self, wi: V, s: F2) -> V {
        Sampler::split_reuse_2d(s, self.ks, |s| {
            let n = V(BeckmannHemisphere::warp(s, self.alpha));
            n * 2. * n.dot(wi) - wi
        }, |s| V(CosineHemisphere::warp(s, ())))
    }

    #[inline(always)] fn pdf(&self, wi: V, wo: V) -> F {
        if Frame::ct(*wo) <= 0. { return 0. }
        let wh = N::v(wi + wo);
        let dp = CosineHemisphere::pdf(*wo, ());
        let sp = self.beckmann(wh) * Frame::ct(**wh) * 0.25 / wh.dot(wo);
        LinearInterp::interp(A2(dp, sp), self.ks)
    }
}
