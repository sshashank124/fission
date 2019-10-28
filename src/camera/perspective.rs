use super::*;


pub struct Perspective {
    fov_scale: F,
}

impl Perspective {
    #[inline]
    pub fn new(fov: F) -> Perspective {
        Perspective {
            fov_scale: (fov / 2.).tand(),
        }
    }

    #[inline]
    pub fn default() -> Perspective {
        Perspective::new(40.)
    }
}

impl CameraModel for Perspective {
    #[inline]
    fn ray_at(&self, P2(x, y): F2) -> R {
        R::unbounded(P::ZERO, V::v(x * self.fov_scale, y * self.fov_scale, 1.))
    }
}
