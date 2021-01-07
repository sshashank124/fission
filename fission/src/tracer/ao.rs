#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

use crate::color::Color;
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
                Color::gray(F::of((0..self.samples).filter(|_| {
                    let wi = V::from(CosineHemisphere::warp(sampler.next_2d()));
                    !scene.intersects(R::r(its.p, f * wi, self.ray_length))
                }).count()) / F::of(self.samples))
            }
        }
    }
}

impl Default for AmbientOcclusion
{ fn default() -> Self { Self { samples: 1, ray_length: F::POS_INF } } }
