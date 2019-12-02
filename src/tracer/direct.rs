use super::*;


pub struct Direct;

impl Direct { #[inline(always)] pub fn new() -> Self { Self } }

impl Trace for Direct {
    #[inline(always)]
    fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
        match scene.intersect(ray) {
            None => scene.lights.iter().map(|light|
                                            light.eval(&ray, None)).sum(),
            Some(its) => {
                let f = its.to_world();
                let wo = f / -ray.d;
                scene.lights.iter().map(|light| {
                    let (r, sray) = light.sample(&its, sampler.next_2d());
                    if r == Color::BLACK || scene.intersects(sray) {
                        Color::BLACK
                    } else {
                        let wi = f / sray.d;
                        r * its.bsdf().eval(wi, wo, its.uv) * wi[Z]
                    }
                }).sum::<Color>() + its.le(&ray)
            }
        }
    }
}
