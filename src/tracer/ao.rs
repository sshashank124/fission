use super::*;

#[derive(Debug)]
pub struct AmbientOcclusion {
    samples: I,
    ray_len: F,
}

impl AmbientOcclusion {
    pub fn new(s: Option<I>, rl: Option<F>) -> Self {
        Self { samples: s.unwrap_or(1), ray_len: rl.unwrap_or(F::POS_INF) }
    }

    #[inline(always)]
    pub fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
        match scene.intersect(ray) {
            None => Color::BLACK,
            Some(its) => {
                let f = its.to_world();
                Color::gray((0..self.samples).filter(|_| {
                    let wi = V(CosineHemisphere::warp(sampler.next_2d(), ()));
                    !scene.intersects(R::r(its.p, f * wi, self.ray_len))
                }).count() as F / self.samples as F)
            }
        }
    }
}
