use super::*;


pub struct Direct;

impl Direct { #[inline(always)] pub fn new() -> Self { Self } }

impl Trace for Direct {
    #[inline(always)]
    fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
        match scene.intersect(ray) {
            None => scene.l_bg(&ray),
            Some(its) => its.le(ray) + {
                let f = its.to_world();
                let wo = f / -ray.d;

                let light = scene.random_light(sampler.next_1d());
                let (le, sray, lpdf) = light.sample(&its, sampler.next_2d());
                let le = Self::if_worth_tracing(le, lpdf,
                                                || if !scene.intersects(sray) {
                    let wi = f / sray.d;
                    le * scene.lights.len() as F
                       * PowerScale::balance(A2(lpdf, its.bpdf(wo, wi)))
                       * its.lb(wo, wi) * Frame::ct(*wi)
                } else { Color::BLACK });

                let (lb, wi, bpdf) = its.sample_lb(wo, sampler.next_2d());
                let sray = R::unbounded(its.p, f * wi);
                le + Self::if_worth_tracing(lb, bpdf, ||
                    scene.intersect(sray).filter(Its::has_emission).map(|its2|
                        lb * its2.le(sray)
                           * PowerScale::balance(A2(bpdf, its2.lpdf(sray)))
                           * Frame::ct(*wi))
                    .unwrap_or(Color::BLACK))
            }
        }
    }
}
