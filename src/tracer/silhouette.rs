use super::*;

pub struct Silhouette;

impl Silhouette {
    pub fn new() -> Self { Self }
}

impl Trace for Silhouette {
    #[inline(always)]
    fn trace(&self, scene: &Scene, _: Sampler, ray: R) -> Color {
        if scene.intersects(ray) {
            Color::BLACK
        } else {
            Color::WHITE
        }
    }
}
