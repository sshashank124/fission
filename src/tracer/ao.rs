use super::*;

use crate::warp;


pub struct AmbientOcclusion {
    samples: I,
    ray_len: F,
}

impl AmbientOcclusion {
    #[inline(always)] pub fn new(s: Option<I>, rl: Option<F>) -> Self {
        Self {
            samples: s.unwrap_or(1),
            ray_len: rl.unwrap_or(F::POS_INF),
        }
    }
}

impl Trace for AmbientOcclusion {
    #[inline(always)]
    fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
        match scene.intersect(ray) {
            None => Color::BLACK,
            Some(its) => {
                let f = its.to_world();
                Color::gray((0..self.samples).filter(|_| {
                    let wi = V(warp::cosine_hemisphere(sampler.next_2d()).0);
                    !scene.intersects(R::r(its.p, f * wi, self.ray_len))
                }).count() as F / self.samples as F)
            }
        }
    }
}
