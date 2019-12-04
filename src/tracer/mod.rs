mod ao;
mod direct;
mod normals;
mod silhouette;

use crate::geometry::*;
use crate::light::*;
use crate::sampler::*;
use crate::scene::Scene;
use crate::shape::*;

pub use ao::AmbientOcclusion;
pub use direct::Direct;
pub use normals::Normals;
pub use silhouette::Silhouette;


pub trait Trace {
    fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color;

    #[inline(always)] fn if_worth_tracing<FN>(l: Color, pdf: F, f: FN) -> Color
        where FN: Fn() -> Color
    { if l != Color::BLACK && pdf > 0. { f() } else { Color::BLACK } }
}

pub enum Tracer {
    AO(AmbientOcclusion),
    Direct(Direct),
    Normals(Normals),
    Silhouette(Silhouette),
}

impl Trace for Tracer {
    fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
        match self {
            Self::AO(t) => t.trace(scene, sampler, ray),
            Self::Direct(t) => t.trace(scene, sampler, ray),
            Self::Normals(t) => t.trace(scene, sampler, ray),
            Self::Silhouette(t) => t.trace(scene, sampler, ray),
        }
    }
}

impl From<AmbientOcclusion> for Tracer
{ #[inline(always)] fn from(t: AmbientOcclusion) -> Self { Self::AO(t) } }

impl From<Direct> for Tracer
{ #[inline(always)] fn from(t: Direct) -> Self { Self::Direct(t) } }

impl From<Normals> for Tracer
{ #[inline(always)] fn from(t: Normals) -> Self { Self::Normals(t) } }

impl From<Silhouette> for Tracer
{ #[inline(always)] fn from(t: Silhouette) -> Self { Self::Silhouette(t) } }
