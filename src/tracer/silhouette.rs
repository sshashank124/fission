use super::*;

#[derive(Debug, Deserialize)]
pub struct Silhouette;

impl Silhouette {
    #[inline(always)] pub fn trace(&self, scene: &Scene, ray: R) -> Color
    { if scene.intersects(ray) { Color::ZERO } else { Color::ONE } }
}
