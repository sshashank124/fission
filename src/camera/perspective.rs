use super::*;
use crate::warp;


pub struct Perspective {
    fov_scale: F,
    lens_radius: F,
    focal_distance: F,
}

impl Perspective {
    #[inline(always)]
    pub fn new(fov: F, lr: Option<F>, fd: Option<F>) -> Self {
        Self {
            fov_scale: F::tand(fov / 2.),
            lens_radius: lr.unwrap_or(0.),
            focal_distance: fd.unwrap_or(0.),
        }
    }
}

impl CameraModel for Perspective {
    #[inline(always)]
    fn ray_at(&self, point: F2, sampler: &mut Sampler) -> R {
        let scaled_point = point * self.fov_scale;
        let dir = V::a2(scaled_point, 1.).unit();

        let ray = R::unbounded(P::ZERO, dir);
        if F::approx_zero(self.lens_radius) { ray }
        else {
            let focus_point = ray.at(self.focal_distance / ray.d[Z]);
            let o = P::a2(warp::uniform_disk(sampler.next_2d()).0
                          * self.lens_radius, 0.);
            R::unbounded(o, focus_point - o)
        }
    }
}
