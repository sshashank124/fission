#[allow(clippy::wildcard_imports)]
use graphite::*;

use crate::color::Color;
use crate::sampler::Sampler;
use crate::scene::Scene;
use crate::shape::intersection::Its;
use crate::util::pdf::Pdf;

#[inline] pub fn l_light(scene: &Scene, its: &Its, wo: V, frame: T, s: F2) -> Color {
    if !its.bsdf().is_delta() {
        let (light, sray) = scene.sample_random_light(its, s);
        if light.pdf > 0. && light.val != Color::ZERO && !scene.intersects(sray) {
            let bsdf = its.bsdf_f_pdf(wo, frame / sray.d);
            if bsdf.pdf > 0. && bsdf.val != Color::ZERO {
                return light.val * bsdf.val * PowerScale::balance2(light.pdf, bsdf.pdf);
            }
        }
    }
    Color::ZERO
}

#[derive(Debug)]
pub struct BounceInfo<'a> {
    pub l:    Color,
    pub tp:   Color,
    pub its:  Option<Its<'a>>,
    pub ray:  R,
    pub spec: bool,
}

#[inline]
pub fn l_mat<'a>(scene: &'a Scene, its: &Its, wo: V, frame: T, s: F2) -> Option<BounceInfo<'a>> {
    let (bsdf, wi, spec) = its.sample_bsdf(wo, s);
    if bsdf.pdf > 0. && bsdf.val != Color::ZERO {
        let ray = its.spawn_ray(frame * wi);
        let its = scene.intersect(ray);
        let light = match &its {
            None => Pdf::sole(scene.lenv(&ray)),
            Some(its) if its.emits() => its.l_emit_pdf(ray),
            _ => Pdf::ZERO,
        };
        let l = if light.pdf > 0. && light.val != Color::ZERO && !spec {
            bsdf.val * light.val * PowerScale::balance2(bsdf.pdf, light.pdf)
        } else { Color::ZERO };
        Some(BounceInfo { l, tp: bsdf.val, its, ray, spec })
    } else { None }
}

#[inline] pub fn trace(scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
    scene.intersect(ray).map_or_else(|| scene.lenv(&ray), |its| {
        let frame = its.to_world();
        let wo = frame / -ray.d;

        its.l_emit(ray) + l_light(scene, &its, wo, frame, sampler.next_2d())
                        + l_mat(scene, &its, wo, frame, sampler.next_2d())
                            .map_or(Color::ZERO, |bounce| bounce.l)
    })
}
