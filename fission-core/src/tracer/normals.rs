#[allow(clippy::wildcard_imports)]
use graphite::*;

use crate::scene::Scene;

#[inline] pub fn trace(scene: &Scene, ray: R) -> Color {
    match scene.intersect(ray) {
        None => Color::ZERO,
        Some(its) => Color::rgb(F3::from(its.n).map(F::abs)),
    }
}
