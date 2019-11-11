pub mod bvh;

use crate::geometry::*;

pub use bvh::BVH;

pub type Mesh = BVH<Triangle>;
