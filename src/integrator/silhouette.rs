use super::*;


pub struct Silhouette;

impl Silhouette {
    #[inline(always)]
    pub fn new() -> Silhouette {
        Silhouette { }
    }
}

impl Integrator for Silhouette {
    #[inline(always)]
    fn sample<S>(&self, scene: &Scene, _sampler: &mut S, ray: R) -> Color
            where S: Sampler {
        if scene.intersects(ray) { Color::BLACK }
        else { Color::WHITE }
    }
}
