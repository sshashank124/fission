#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

use crate::color::Color;
use crate::util::pdf::PDF;

use super::fresnel;

#[derive(Debug, Deserialize)]
pub struct Dielectric {
    #[serde(rename="ior", deserialize_with="de_ior_eta")]
    eta: F,
}

impl Dielectric {
    #[inline]
    pub fn sample(&self, wi: V, s: F2) -> (PDF<Color>, V, bool) {
        let (fr, ctt, eta) = fresnel::eval(Frame::ct(wi), self.eta);
        let (wo, p) = if s[0] <= fr { (V::from(Frame::reflect(wi)), fr) }
                      else { (V::from(A3(-eta * wi[X], -eta * wi[Y], ctt)).unit(), 1. - fr) };
        (PDF::new(Color::ONE, p), wo, true)
    }
}

fn de_ior_eta<'de, D>(de: D) -> Result<F, D::Error>
where D: serde::Deserializer<'de>
{ F2::deserialize(de).map(fresnel::eta) }
