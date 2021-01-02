use super::*;

pub struct Silhouette;

impl Silhouette {
    #[inline] pub fn trace(scene: &Scene, ray: R) -> Color
    { if scene.intersects(ray) { Color::ZERO } else { Color::ONE } }
}
