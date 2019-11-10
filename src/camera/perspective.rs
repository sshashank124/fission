use super::*;


pub struct Perspective {
    fov_scale: F,
}

impl Perspective {
    #[inline(always)]
    pub fn new(fov: F) -> Perspective {
        Perspective {
            fov_scale: (fov / 2.).tand(),
        }
    }
}

impl CameraModel for Perspective {
    #[inline(always)]
    fn ray_at(&self, P2(x, y): F2) -> R {
        R::unbounded(P::ZERO, V::v(x * self.fov_scale, y * self.fov_scale, 1.))
    }
}
