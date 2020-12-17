mod perspective;

use crate::prelude::*;
use crate::image::*;
use crate::sampler::*;

pub use perspective::Perspective;

#[derive(Debug)]
pub enum CameraType {
    Perspective(Perspective),
}

#[derive(Debug)]
pub struct Camera {
    pub resolution: I2,
    model:          CameraType,
    to_world:       T,
    from_pixel:     T2,
}

impl Camera {
    pub fn new(model: CameraType, resolution: I2, to_world: T) -> Self {
        Self { from_pixel: T2::scale(A2(2., -2.) / resolution[Y] as F)
                           * T2::translate(F2::from(resolution) * -0.5),
               resolution,
               model,
               to_world }
    }

    #[inline(always)]
    pub fn ray_at(&self, point: F2, sampler: &mut Sampler) -> R {
        self.to_world * self.model.ray_at(self.from_pixel * point, sampler)
    }

    pub fn new_image(&self) -> Image { Image::new(self.resolution) }
}

impl CameraType {
    #[inline(always)]
    fn ray_at(&self, point: F2, sampler: &mut Sampler) -> R {
        match self {
            Self::Perspective(c) => c.ray_at(point, sampler),
        }
    }
}

impl From<Perspective> for CameraType {
    fn from(p: Perspective) -> Self { Self::Perspective(p) }
}
