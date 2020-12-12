use super::*;

pub struct Silhouette;

impl Silhouette {
    pub fn new() -> Self { Self }

    #[inline(always)]
    pub fn trace(&self, scene: &Scene, ray: R) -> Color {
        if scene.intersects(ray) {
            Color::BLACK
        } else {
            Color::WHITE
        }
    }
}
