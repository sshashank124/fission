mod perspective;

use crate::geometry::*;
pub use perspective::Perspective;


pub enum CameraType {
    Perspective(Perspective),
}

trait CameraModel {
    fn ray_at(&self, point: F2) -> R;
}

pub struct Camera {
    resolution: I2,
    model: CameraType,
    to_world: T,
    from_pixel: T2,
}

impl Camera {
    #[inline(always)]
    pub fn new<C>(model: C, resolution: I2, to_world: T) -> Camera
            where C: Into<CameraType> {
        Camera {
            from_pixel: T2::scale(P2(2., -2.) / resolution.1 as F) *
                        T2::translate(resolution / -2.),
            model: model.into(),
            resolution,
            to_world,
        }
    }

    #[inline(always)]
    pub fn default() -> Camera {
        Camera::new(Perspective::default(), P2(1280, 720), T::I)
    }

    #[inline(always)]
    pub fn ray_at(&self, point: F2) -> R {
        self.to_world * self.model.ray_at(self.from_pixel * point)
    }

    #[inline(always)]
    pub fn resolution(&self) -> I2 {
        self.resolution
    }
}

impl CameraModel for CameraType {
    #[inline(always)]
    fn ray_at(&self, point: F2) -> R {
        match self {
            CameraType::Perspective(model) => model.ray_at(point),
        }
    }
}

impl From<Perspective> for CameraType {
    #[inline(always)]
    fn from(perspective: Perspective) -> CameraType {
        CameraType::Perspective(perspective)
    }
}
