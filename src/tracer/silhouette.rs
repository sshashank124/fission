use super::*;


pub struct Silhouette;

impl Silhouette { pub fn new() -> Self { Self } }

impl Trace for Silhouette {
    #[inline(always)]
    fn trace(&self, scene: &Scene, _: Sampler, ray: R) -> Color
    { A2(Color::WHITE, Color::BLACK)[scene.intersects(ray)] }
}
