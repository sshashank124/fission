#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

use crate::color::Color;
use crate::sampler;
use crate::util::pdf::Pdf;

use super::fresnel;

#[derive(Debug, Deserialize)]
#[serde(from="MicrofacetConfig")]
pub struct Microfacet {
    kd:    Color,
    ks:    F,
    alpha: F,
    eta:   F,
}

impl Microfacet {
    #[inline] fn beckmann(&self, v: V) -> F {
        F::exp(-Frame::t2t(v) / self.alpha.sq()) * F::INV_PI
        / (self.alpha * Frame::c2t(v)).sq()
    }

    #[inline] fn smith_beckmann_g1(&self, v: V, n: V) -> F {
        let tt = Frame::tt(v);
        if tt == 0. { return 1. }
        if F3::dot(n.conv(), v.conv()) * Frame::ct(v) <= 0. { return 0. }
        let a = (self.alpha * tt).inv();
        if a >= 1.6 { return 1. }
        a.mul_add(3.535, 2.181 * a.sq())
            / a.sq().mul_add(2.577, a.mul_add(2.276, 1.))
    }

    #[inline] pub fn eval(&self, wi: V, wo: V) -> Color {
        let ct_i = Frame::ct(wi);
        let ct_o = Frame::ct(wo);
        if ct_i <= 0. || ct_o <= 0. { return Color::ZERO }
        let wh = (wi + wo).unit();
        let beck = self.beckmann(wh);
        let fr = fresnel::eval(F3::dot(wh.conv(), wi.conv()), self.eta).0;
        let g = self.smith_beckmann_g1(wi, wh) * self.smith_beckmann_g1(wo, wh);
        self.kd * F::INV_PI * ct_o + (self.ks * beck * fr * g * 0.25) / ct_i
    }

    #[inline]
    pub fn sample(&self, wi: V, s: F2) -> (Pdf<Color>, V, bool) {
        let spec = |s| {
            let n = conv!(BeckmannHemisphere::warp(s, self.alpha) => V);
            (n * 2. * F3::dot(n.conv(), wi.conv()) - wi).unit()
        };
        let diffuse = |s| conv!(CosineHemisphere::warp(s) => V);
        let wo = sampler::split_reuse_2d(s, self.ks, spec, diffuse);
        let p = self.pdf(wi, wo);
        let color = if p <= 0. { Color::ZERO } else { self.eval(wi, wo) / p };
        (Pdf::new(color, p), wo, false)
    }

    #[inline] pub fn pdf(&self, wi: V, wo: V) -> F {
        let wh = (wi + wo).unit();
        let dp = CosineHemisphere::pdf(wo);
        let sp = self.beckmann(wh) * Frame::ct(wh) * 0.25 / F3::dot(wh.conv(), wo.conv());
        LinearScale::interp(A2(dp, sp), self.ks)
    }
}


#[derive(Debug, Deserialize)]
struct MicrofacetConfig {
    kd: Color,
    alpha: Option<F>,
    ior: Option<F2>,
}

impl From<MicrofacetConfig> for Microfacet {
    fn from(mc: MicrofacetConfig) -> Self {
        Self {
            kd: mc.kd,
            ks: 1. - mc.kd.max_channel(),
            alpha: mc.alpha.unwrap_or(0.1),
            eta: fresnel::eta(mc.ior.unwrap_or(A2(1.000_277, 1.5046))),
        }
    }
}
