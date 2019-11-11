mod perspective;

use crate::geometry::*;
use crate::sampler::*;
pub use perspective::Perspective;


pub enum CameraType {
    Perspective(Perspective),
}

pub trait CameraModel {
    fn ray_at(&self, point: F2, sampler: &mut Sampler) -> R;
}

pub struct Camera {
    pub resolution: I2,
    model: CameraType,
    to_world: T,
    from_pixel: T2,
}

impl Camera {
    #[inline(always)]
    pub fn new<C>(model: C, resolution: I2, to_world: T) -> Camera
            where C: Into<CameraType> {
        Camera {
            from_pixel: T2::scale(P2(2., -2.) / resolution[Y] as F) *
                        T2::translate(resolution / -2.),
            model: model.into(),
            resolution,
            to_world,
        }
    }
}

impl CameraModel for Camera {
    #[inline(always)]
    fn ray_at(&self, point: F2, sampler: &mut Sampler) -> R {
        self.to_world * self.model.ray_at(self.from_pixel * point,
                                          sampler)
    }
}

impl CameraModel for CameraType {
    #[inline(always)]
    fn ray_at(&self, point: F2, sampler: &mut Sampler) -> R {
        match self {
            CameraType::Perspective(model) => model.ray_at(point, sampler),
        }
    }
}

impl From<Perspective> for CameraType {
    #[inline(always)]
    fn from(perspective: Perspective) -> CameraType {
        CameraType::Perspective(perspective)
    }
}
