mod ao;
mod direct_ems;
mod direct_mats;
mod normals;
mod silhouette;

use crate::geometry::*;
use crate::light::*;
use crate::sampler::*;
use crate::scene::Scene;
use crate::shape::*;

pub use ao::AmbientOcclusion;
pub use direct_ems::DirectEms;
pub use direct_mats::DirectMats;
pub use normals::Normals;
pub use silhouette::Silhouette;


pub trait Trace {
    fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color;
}

pub enum Tracer {
    AO(AmbientOcclusion),
    DirectEms(DirectEms),
    DirectMats(DirectMats),
    Normals(Normals),
    Silhouette(Silhouette),
}

impl Trace for Tracer {
    fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
        match self {
            Self::AO(t) => t.trace(scene, sampler, ray),
            Self::DirectEms(t) => t.trace(scene, sampler, ray),
            Self::DirectMats(t) => t.trace(scene, sampler, ray),
            Self::Normals(t) => t.trace(scene, sampler, ray),
            Self::Silhouette(t) => t.trace(scene, sampler, ray),
        }
    }
}

impl From<AmbientOcclusion> for Tracer
{ #[inline(always)] fn from(t: AmbientOcclusion) -> Self { Self::AO(t) } }

impl From<DirectEms> for Tracer
{ #[inline(always)] fn from(t: DirectEms) -> Self { Self::DirectEms(t) } }

impl From<DirectMats> for Tracer
{ #[inline(always)] fn from(t: DirectMats) -> Self { Self::DirectMats(t) } }

impl From<Normals> for Tracer
{ #[inline(always)] fn from(t: Normals) -> Self { Self::Normals(t) } }

impl From<Silhouette> for Tracer
{ #[inline(always)] fn from(t: Silhouette) -> Self { Self::Silhouette(t) } }
