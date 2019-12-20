mod ao;
mod direct;
mod normals;
mod path;
mod silhouette;

use crate::bsdf::*;
use crate::geometry::*;
use crate::light::*;
use crate::sampler::*;
use crate::scene::Scene;
use crate::shape::*;

pub use ao::AmbientOcclusion;
pub use direct::Direct;
pub use normals::Normals;
pub use path::Path;
pub use silhouette::Silhouette;

pub trait Trace {
    fn trace(&self, scene: &Scene, sampler: Sampler, ray: R) -> Color;
}

pub enum Tracer {
    AO(AmbientOcclusion),
    Direct(Direct),
    Normals(Normals),
    Path(Path),
    Silhouette(Silhouette),
}

impl Trace for Tracer {
    fn trace(&self, scene: &Scene, sampler: Sampler, ray: R) -> Color {
        match self {
            Self::AO(t) => t.trace(scene, sampler, ray),
            Self::Direct(t) => t.trace(scene, sampler, ray),
            Self::Normals(t) => t.trace(scene, sampler, ray),
            Self::Path(t) => t.trace(scene, sampler, ray),
            Self::Silhouette(t) => t.trace(scene, sampler, ray),
        }
    }
}

impl From<AmbientOcclusion> for Tracer {
    fn from(t: AmbientOcclusion) -> Self { Self::AO(t) }
}

impl From<Direct> for Tracer {
    fn from(t: Direct) -> Self { Self::Direct(t) }
}

impl From<Normals> for Tracer {
    fn from(t: Normals) -> Self { Self::Normals(t) }
}

impl From<Path> for Tracer {
    fn from(t: Path) -> Self { Self::Path(t) }
}

impl From<Silhouette> for Tracer {
    fn from(t: Silhouette) -> Self { Self::Silhouette(t) }
}
