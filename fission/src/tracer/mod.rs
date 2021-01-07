mod ao;
mod direct;
mod normals;
mod path;
mod silhouette;

#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

use crate::color::Color;
use crate::sampler::Sampler;
use crate::scene::Scene;

use ao::AmbientOcclusion;
use path::Path;

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
    #[inline]
    pub fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
        match self {
            Self::AmbientOcclusion(t) => t.trace(scene, sampler, ray),
            Self::Direct => direct::trace(scene, sampler, ray),
            Self::Normals => normals::trace(scene, ray),
            Self::Path(t) => t.trace(scene, sampler, ray),
            Self::Silhouette => silhouette::trace(scene, ray),
        }
    }
}

impl From<AmbientOcclusion> for Tracer
{ fn from(t: AmbientOcclusion) -> Self { Self::AmbientOcclusion(t) } }

impl From<Path> for Tracer
{ fn from(t: Path) -> Self { Self::Path(t) } }
