use super::*;
use crate::warp;


pub struct Perspective {
    fov_scale: F,
    lens_radius: F,
    focal_distance: F,
}

impl Perspective {
    #[inline(always)]
    pub fn new(fov: F,
               lens_radius: Option<F>,
               focal_distance: Option<F>) -> Perspective {
        Perspective {
            fov_scale: (fov / 2.).tand(),
            lens_radius: lens_radius.unwrap_or(0.),
            focal_distance: focal_distance.unwrap_or(0.),
        }
    }
}

impl CameraModel for Perspective {
    #[inline(always)]
    fn ray_at(&self, A2(x, y): F2, sampler: &mut Sampler) -> R {
        let v = V::v(x * self.fov_scale, y * self.fov_scale, 1.).unit();
        let ray = R::unbounded(P::ZERO, v);

        if self.lens_radius < F::EPSILON { ray }
        else {
            let sp = warp::uniform_disk(sampler.next_2d()) * self.lens_radius;
            let focus_point = ray.at(self.focal_distance * ray.d_inv[Z]);
            let o = P::p(sp[X], sp[Y], 0.);
            R::unbounded(o, focus_point - o)
        }
    }
}
