use super::*;


pub struct Perspective {
    fov_scale: F,
    lens_r: F,
    fd: F,
}

impl Perspective {
    pub fn new(fov: F, lr: Option<F>, fd: Option<F>) -> Self {
        let fov_scale = F::tand(0.5 * fov);
        let lens_r = lr.unwrap_or(0.);
        let fd = fd.unwrap_or(0.);
        Self { fov_scale, lens_r, fd }
    }
}

impl CameraModel for Perspective {
    #[inline(always)] fn ray_at(&self, point: F2, sampler: &mut Sampler) -> R {
        let ray = R::unbounded(P::ZERO, V::a2(point * self.fov_scale, 1.));
        if F::approx_zero(self.lens_r) { ray } else {
            let focus_point = ray.at(self.fd / ray.d[Z]);
            let o = P::a2(UniformDisk::warp(sampler.next_2d(), ())
                          * self.lens_r, 0.);
            R::unbounded(o, focus_point - o)
        }
    }
}
