#[allow(clippy::wildcard_imports)]
use graphite::*;

use crate::scene::Scene;

#[inline] pub fn trace(scene: &Scene, ray: R) -> Color
{ if scene.intersects(ray) { Color::ZERO } else { Color::ONE } }
