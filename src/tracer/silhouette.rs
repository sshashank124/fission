use super::*;


pub struct Silhouette;

impl Silhouette { #[inline(always)] pub fn new() -> Self { Self { } } }

impl Trace for Silhouette {
    #[inline(always)]
    fn trace(&self, scene: &Scene, _: &mut Sampler, ray: R) -> Color {
        if scene.intersects(ray) { Color::BLACK } else { Color::WHITE }
    }
}
