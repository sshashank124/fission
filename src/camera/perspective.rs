#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

use crate::sampler::Sampler;

#[derive(Debug, Deserialize)]
pub struct Perspective {
    #[serde(rename="fov", deserialize_with="de_fov_scale")]
    fov_scale:      F,
    #[serde(default)]
    lens_radius:    F,
    #[serde(default)]
    focal_distance: F,
}

impl Perspective {
    #[inline]
    pub fn ray_at(&self, point: F2, sampler: &mut Sampler) -> R {
        let d = V::from(F3::a2a(point * self.fov_scale, 1.));
        let ray = R::unbounded(P::ZERO, d);
        if F::approx_zero(self.lens_radius) { ray } else {
            let focus_point = ray.at(self.focal_distance / ray.d[Z]);
            let o = P::from(F3::a2a(UniformDisk::warp(sampler.next_2d()) * self.lens_radius, 0.));
            R::unbounded(o, focus_point - o)
        }
    }
}


fn de_fov_scale<'de, D>(de: D) -> Result<F, D::Error> where D: serde::Deserializer<'de>
{ F::deserialize(de).map(|fov| F::tand(0.5 * fov)) }
