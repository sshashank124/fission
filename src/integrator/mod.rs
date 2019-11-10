mod av;
mod normals;
mod silhouette;

use crate::geometry::*;
use crate::sampler::Sampler;
use crate::scene::Scene;
use crate::structure::*;


pub trait Integrator {
    fn sample<S>(&self, scene: &Scene, sampler: &mut S, ray: R) -> Color
        where S: Sampler;
}

pub use av::AverageVisibility;
pub use normals::Normals;
pub use silhouette::Silhouette;
