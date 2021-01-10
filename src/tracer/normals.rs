#[allow(clippy::wildcard_imports)]
use graphite::*;

use crate::color::Color;
use crate::scene::Scene;

#[inline] pub fn trace(scene: &Scene, ray: R) -> Color {
    match scene.intersect(ray) {
        None => Color::ZERO,
        Some(its) => Color::from_rgb(F3::from(its.n).map(F::abs)),
    }
}
