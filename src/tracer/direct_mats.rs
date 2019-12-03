use super::*;


pub struct DirectMats;

impl DirectMats { #[inline(always)] pub fn new() -> Self { Self } }

impl Trace for DirectMats {
    #[inline(always)]
    fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
        match scene.intersect(ray) {
            None => scene.l_bg(&ray),
            Some(its) => its.le(ray) + {
                let f = its.to_world();
                let (rb, wo) = its.sample_lb(f / -ray.d, sampler.next_2d());
                let ray2 = R::unbounded(its.p, f * wo);
                match scene.intersect(ray2) {
                    None => Color::BLACK,
                    Some(its2) => rb * its2.le(ray2)
                }
            }
        }
    }
}
