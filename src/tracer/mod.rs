mod av;
mod heatmap;
mod normals;
mod silhouette;

use crate::geometry::*;
use crate::sampler::*;
use crate::scene::Scene;
use crate::shape::*;

pub use av::AverageVisibility;
pub use heatmap::HeatMap;
pub use normals::Normals;
pub use silhouette::Silhouette;


pub trait Trace {
    fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color;
}

pub enum Tracer {
    AV(AverageVisibility),
    HeatMap(HeatMap),
    Normals(Normals),
    Silhouette(Silhouette),
}

impl Trace for Tracer {
    fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
        match self {
            Self::AV(t) => t.trace(scene, sampler, ray),
            Self::HeatMap(t) => t.trace(scene, sampler, ray),
            Self::Normals(t) => t.trace(scene, sampler, ray),
            Self::Silhouette(t) => t.trace(scene, sampler, ray),
        }
    }
}

impl From<AverageVisibility> for Tracer {
    #[inline(always)]
    fn from(t: AverageVisibility) -> Self { Self::AV(t) }
}

impl From<HeatMap> for Tracer {
    #[inline(always)]
    fn from(t: HeatMap) -> Self { Self::HeatMap(t) }
}

impl From<Normals> for Tracer {
    #[inline(always)]
    fn from(t: Normals) -> Self { Self::Normals(t) }
}

impl From<Silhouette> for Tracer {
    #[inline(always)]
    fn from(t: Silhouette) -> Self { Self::Silhouette(t) }
}
