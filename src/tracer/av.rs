use super::*;

use crate::warp;


pub struct AverageVisibility {
    ray_len: F,
}

impl AverageVisibility {
    #[inline(always)]
    pub fn new(ray_len: F) -> AverageVisibility {
        AverageVisibility { ray_len }
    }
}

impl Trace for AverageVisibility {
    #[inline(always)]
    fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R)
            -> Color {
        match scene.intersect(ray) {
            None => Color::WHITE,
            Some(its) => {
                let dir = T::from_dir(*its.n.unitn())
                        * V(warp::uniform_hemisphere(sampler.next_2d()));
                let ray = R::new(its.p, dir, B::with_ceil(self.ray_len));
                if scene.intersects(ray) { Color::BLACK }
                else { Color::WHITE }
            }
        }
    }
}
