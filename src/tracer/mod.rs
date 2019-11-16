mod av;
mod normals;
mod silhouette;

use crate::geometry::*;
use crate::sampler::*;
use crate::scene::Scene;
use crate::structure::*;

pub use av::AverageVisibility;
pub use normals::Normals;
pub use silhouette::Silhouette;


pub trait Trace {
    fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color;
}

pub enum Tracer {
    AV(AverageVisibility),
    Normals(Normals),
    Silhouette(Silhouette),
}

impl Trace for Tracer {
    fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
        match self {
            Self::AV(i) => i.trace(scene, sampler, ray),
            Self::Normals(i) => i.trace(scene, sampler, ray),
            Self::Silhouette(i) => i.trace(scene, sampler, ray),
        }
    }
}

impl From<AverageVisibility> for Tracer {
    #[inline(always)]
    fn from(i: AverageVisibility) -> Self { Self::AV(i) }
}

impl From<Normals> for Tracer {
    #[inline(always)]
    fn from(i: Normals) -> Self { Self::Normals(i) }
}

impl From<Silhouette> for Tracer {
    #[inline(always)]
    fn from(i: Silhouette) -> Self { Self::Silhouette(i) }
}
