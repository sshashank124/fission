use super::*;

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
                Color::gray((0..self.samples).filter(|_| {
                    let wi = V::from(CosineHemisphere::warp(sampler.next_2d()));
                    !scene.intersects(R::r(its.p, f * wi, self.ray_length))
                }).count() as F / self.samples as F)
            }
        }
    }
}

impl Default for AmbientOcclusion
{ fn default() -> Self { Self { samples: 1, ray_length: F::POS_INF } } }
