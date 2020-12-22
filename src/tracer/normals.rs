use super::*;

#[derive(Debug)]
pub struct Normals;

impl Normals {
    pub fn new() -> Self { Self }

    #[inline(always)] pub fn trace(&self, scene: &Scene, ray: R) -> Color {
        match scene.intersect(ray) {
            None => Color::ZERO,
            Some(its) => Color::rgb(F3::from(its.n).map(F::abs)),
        }
    }
}
