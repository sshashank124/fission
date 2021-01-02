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

#[derive(Debug, Deserialize)]
#[serde(tag="type", rename_all="snake_case")]
pub enum Tracer {
    AmbientOcclusion(AmbientOcclusion),
    Direct,
    Normals,
    Path(Path),
    Silhouette,
}

impl Tracer {
    #[inline(always)]
    pub fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
        match self {
            Self::AmbientOcclusion(t) => t.trace(scene, sampler, ray),
            Self::Direct => Direct::trace(scene, sampler, ray),
            Self::Normals => Normals::trace(scene, ray),
            Self::Path(t) => t.trace(scene, sampler, ray),
            Self::Silhouette => Silhouette::trace(scene, ray),
        }
    }
}

impl From<AmbientOcclusion> for Tracer
{ fn from(t: AmbientOcclusion) -> Self { Self::AmbientOcclusion(t) } }

impl From<Direct> for Tracer
{ fn from(_: Direct) -> Self { Self::Direct } }

impl From<Normals> for Tracer
{ fn from(_: Normals) -> Self { Self::Normals } }

impl From<Path> for Tracer
{ fn from(t: Path) -> Self { Self::Path(t) } }

impl From<Silhouette> for Tracer
{ fn from(_: Silhouette) -> Self { Self::Silhouette } }
