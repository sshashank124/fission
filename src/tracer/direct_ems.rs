use super::*;


pub struct DirectEms;

impl DirectEms { #[inline(always)] pub fn new() -> Self { Self } }

impl Trace for DirectEms {
    #[inline(always)]
    fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
        match scene.intersect(ray) {
            None => scene.l_bg(&ray),
            Some(its) => its.le(ray) + {
                let f = its.to_world();
                let (r, sray) = scene.random_light(sampler.next_1d())
                                     .sample(&its, sampler.next_2d());
                if r == Color::BLACK || scene.intersects(sray) { Color::BLACK }
                else {
                    let wi = f / sray.d;
                    r * its.lb(wi, f / -ray.d)
                      * Frame::ct(*wi)
                      * scene.lights.len() as F
                }
            }
        }
    }
}
