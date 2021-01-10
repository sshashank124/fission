mod perspective;

#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

use crate::sampler::Sampler;

use perspective::Perspective;

#[derive(Debug, Deserialize)]
#[serde(from="CameraConfig")]
pub struct Camera {
        model:      Type,
    pub resolution: I2,
        from_pixel: T2,
        to_world:   T,
}

#[derive(Debug, Deserialize)]
#[serde(tag="type", rename_all="snake_case")]
enum Type {
    Perspective(Perspective),
}

impl Camera {
    #[inline]
    pub fn ray_at(&self, point: F2, sampler: &mut Sampler) -> R
    { self.to_world * self.model.ray_at(self.from_pixel * point, sampler) }
}

impl Type {
    #[inline] fn ray_at(&self, point: F2, sampler: &mut Sampler) -> R {
        match self {
            Self::Perspective(c) => c.ray_at(point, sampler),
        }
    }
}

impl From<Perspective> for Type
{ fn from(p: Perspective) -> Self { Self::Perspective(p) } }


#[derive(Debug, Deserialize)]
struct CameraConfig {
    #[serde(flatten)]
    model:      Type,
    resolution: I2,
    transforms: Vec<T>,
}

impl From<CameraConfig> for Camera {
    fn from(cc: CameraConfig) -> Self {
        Self {
            from_pixel: T2::scale(A2(2., -2.) / F::of(cc.resolution[Y]))
                      * T2::translate(F2::of(cc.resolution) / -2.),
            resolution: cc.resolution,
            model: cc.model,
            to_world: T::product(cc.transforms.into_iter()) }
    }
}
