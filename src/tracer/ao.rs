#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

use crate::color::{Color, Rgb};
use crate::sampler::Sampler;
use crate::scene::Scene;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct AmbientOcclusion {
    samples:    I,
    ray_length: F,
}

impl AmbientOcclusion {
    #[inline]
    pub fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
        match scene.intersect(ray) {
            None => Color::ZERO,
            Some(its) => {
                let f = its.to_world();
                let num_escaped = (0..self.samples).filter(|_| {
                    let wi = conv!(CosineHemisphere::warp(sampler.next_2d()) => V);
                    !scene.intersects(R::r(its.p, f * wi, self.ray_length))
                }).count();
                conv!(conv!(num_escaped => F) / conv!(self.samples => F) => Rgb => Color)
            }
        }
    }
}

impl Default for AmbientOcclusion
{ fn default() -> Self { Self { samples: 1, ray_length: F::POS_INF } } }
