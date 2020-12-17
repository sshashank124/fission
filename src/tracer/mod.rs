mod ao;
mod direct;
mod normals;
mod path;
mod silhouette;

use crate::prelude::*;
use crate::sampler::*;
use crate::scene::Scene;
use crate::shape::*;

pub use ao::AmbientOcclusion;
pub use direct::Direct;
pub use normals::Normals;
pub use path::Path;
pub use silhouette::Silhouette;

#[derive(Debug)]
pub enum Tracer {
    AO(AmbientOcclusion),
    Direct(Direct),
    Normals(Normals),
    Path(Path),
    Silhouette(Silhouette),
}

impl Tracer {
    pub fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
        match self {
            Self::AO(t) => t.trace(scene, sampler, ray),
            Self::Direct(t) => t.trace(scene, sampler, ray),
            Self::Normals(t) => t.trace(scene, ray),
            Self::Path(t) => t.trace(scene, sampler, ray),
            Self::Silhouette(t) => t.trace(scene, ray),
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
