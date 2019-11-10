mod av;
mod normals;
mod silhouette;

use crate::geometry::*;
use crate::sampler::Sampler;
use crate::scene::Scene;
use crate::structure::*;

pub use av::AverageVisibility;
pub use normals::Normals;
pub use silhouette::Silhouette;


pub enum Integrator {
    AV(AverageVisibility),
    Normals(Normals),
    Silhouette(Silhouette),
}

pub trait Integrate {
    fn sample(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color;
}

impl Integrate for Integrator {
    fn sample(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
        match self {
            Integrator::AV(i) => i.sample(scene, sampler, ray),
            Integrator::Normals(i) => i.sample(scene, sampler, ray),
            Integrator::Silhouette(i) => i.sample(scene, sampler, ray),
        }
    }
}

impl From<AverageVisibility> for Integrator {
    #[inline(always)]
    fn from(i: AverageVisibility) -> Integrator { Integrator::AV(i) }
}

impl From<Normals> for Integrator {
    #[inline(always)]
    fn from(i: Normals) -> Integrator { Integrator::Normals(i) }
}

impl From<Silhouette> for Integrator {
    #[inline(always)]
    fn from(i: Silhouette) -> Integrator { Integrator::Silhouette(i) }
}
