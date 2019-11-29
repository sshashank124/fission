use super::*;


pub struct Direct;

impl Direct
{ #[inline(always)] pub fn new() -> Self { Self } }

impl Trace for Direct {
    #[inline(always)]
    fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
        match scene.intersect(ray) {
            None => Color::BLACK,
            Some(its) => {
                let f = its.to_world();
                let wo = f / -ray.d;
                scene.lights.iter().filter_map(|light| {
                    let (r, sray) = light.sample(its.p, sampler.next_2d());
                    if scene.intersects(sray) { None }
                    else {
                        let wi = f / sray.d;
                        Some(r * its.bsdf().eval((wi, wo, its.uv)) * wi[Z])
                    }
                }).sum::<Color>()
            }
        }
    }
}
