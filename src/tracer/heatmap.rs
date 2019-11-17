use super::*;


pub struct HeatMap {
    scale: F,
}

impl HeatMap {
    #[inline(always)] pub fn new(scale: F) -> Self { Self { scale } }
}

impl Trace for HeatMap {
    #[inline(always)]
    fn trace(&self, scene: &Scene, _: &mut Sampler, ray: R)
            -> Color {
        match scene.intersect(ray) {
            None => Color::BLACK,
            Some(its) => {
                let i = F::sqrt(its.i as F) / self.scale;
                Color::rgb(i, i, i)
            }
        }
    }
}
