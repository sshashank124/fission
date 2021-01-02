use super::*;

pub struct Normals;

impl Normals {
    #[inline(always)] pub fn trace(scene: &Scene, ray: R) -> Color {
        match scene.intersect(ray) {
            None => Color::ZERO,
            Some(its) => Color::rgb(F3::from(its.n).map(F::abs)),
        }
    }
}
