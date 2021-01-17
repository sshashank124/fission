#[allow(clippy::wildcard_imports)]
use graphite::*;

use crate::color::{Color, RGB};
use crate::scene::Scene;

#[inline] pub fn trace(scene: &Scene, ray: R) -> Color {
    match scene.intersect(ray) {
        None => Color::ZERO,
        Some(its) => {
            let abs_dir = conv!(its.n => F3).map(F::abs);
            conv!(abs_dir => RGB => Color)
        }
    }
}
