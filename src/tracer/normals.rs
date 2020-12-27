use super::*;

#[derive(Debug, Deserialize)]
pub struct Normals;

impl Normals {
    #[inline(always)] pub fn trace(&self, scene: &Scene, ray: R) -> Color {
        match scene.intersect(ray) {
            None => Color::ZERO,
            Some(its) => Color::rgb(F3::from(its.n).map(F::abs)),
        }
    }
}
