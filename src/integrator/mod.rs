mod av;
mod normals;
mod silhouette;

use crate::geometry::*;
use crate::scene::Scene;
use crate::structure::*;


pub trait Integrator {
    fn sample(&self, scene: &Scene, ray: R) -> Color;
}

pub use av::AverageVisibility;
pub use normals::Normals;
pub use silhouette::Silhouette;
