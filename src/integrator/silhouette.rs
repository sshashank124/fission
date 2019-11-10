use super::*;


pub struct Silhouette;

impl Silhouette {
    #[inline(always)]
    pub fn new() -> Silhouette {
        Silhouette { }
    }
}

impl Integrate for Silhouette {
    #[inline(always)]
    fn sample(&self, scene: &Scene, _sampler: &mut Sampler, ray: R) -> Color {
        if scene.intersects(ray) { Color::BLACK }
        else { Color::WHITE }
    }
}
