use super::*;


pub struct Direct;

impl Direct {
    pub fn new() -> Self { Self }

    #[inline(always)]
    pub fn li<'a>(scene: &'a Scene, sampler: &mut Sampler, its: &Its<'a>,
                  ray: &R, media: bool)
            -> (Color, Option<(Color, R<'a>, Its<'a>, bool)>) {
        let frame = its.to_world();
        let wo = frame / -ray.d;

        let ll = Self::ll(scene, sampler, its, frame, wo, media);
        let (ls, details) = Self::ls(scene, sampler, its, frame, wo, media);

        (ll + ls, details)
    }

    #[inline(always)]
    pub fn ll(scene: &Scene, sampler: &mut Sampler, its: &Its, frame: T,
              wo: V, media: bool) -> Color {
        if !its.bsdf().is_delta() {
            let light = scene.random_light(sampler.next_1d());
            let (le, sray, lpdf) = light.sample(&its, sampler.next_2d());
            if lpdf > 0. && le != Color::ZERO {
                let (ls, spdf) = match its.phase {
                    None => its.lb_pdf(wo, frame / sray.d),
                    Some(phase_fn) => {
                        // let p = phase_fn.eval(wo, frame / sray.d);
                        let p = phase_fn.eval(frame * wo, sray.d);
                        (Color::gray(p), p)
                    }
                };
                if ls != Color::ZERO {
                    let ltr = if media {
                        let (its, tr) = scene.intersect_tr(sray);
                        its.map(|_| Color::ZERO).unwrap_or(tr)
                    } else if scene.intersects(sray) { Color::ZERO }
                    else { Color::ONE };
                    if ltr != Color::ZERO {
                        return le * ls * ltr * scene.lights.len() as F
                                  * PowerScale::balance2(lpdf, spdf)
                    }
                }
            }
        }
        Color::ZERO
    }

    #[inline(always)]
    pub fn ls<'a>(scene: &'a Scene, sampler: &mut Sampler, its: &Its<'a>,
                  frame: T, wo: V, media: bool)
            -> (Color, Option<(Color, R<'a>, Its<'a>, bool)>) {
        let (ls, wi, spdf, spec) = match its.phase {
            None => its.sample_lb(wo, sampler.next_2d()),
            Some(phase_fn) => {
                let (p, wi) = phase_fn.sample(wo, sampler.next_2d());
                (Color::gray(p), wi, p, false)
            }
        };
        if spdf <= 0. || ls == Color::ZERO { return (Color::ZERO, None) }

        let mut ray = its.ray(frame * wi);
        if its.phase.is_some() { ray.m = its.medium_towards(ray.d); }
        let (its, tr) = if media { scene.intersect_tr(ray) }
                        else { (scene.intersect(ray), Color::ONE) };

        let (le, lpdf) = if let Some(ref its) = its {
            if its.has_emission() { (its.le(ray), its.lpdf(ray)) }
            else { (Color::ZERO, 0.) }
        } else { (Color::ZERO, 0.) };

        let ld = if lpdf > 0. && le != Color::ZERO && !spec {
            ls * le * tr * PowerScale::balance2(spdf, lpdf)
        } else { Color::ZERO };

        (ld, its.map(|i| (ls, ray, i, spec)))
    }
}

impl Trace for Direct {
    #[inline(always)]
    fn trace(&self, scene: &Scene, mut sampler: Sampler, ray: R) -> Color {
        match scene.intersect(ray) {
            None => scene.lenv(&ray),
            Some(its) => its.le(ray)
                       + Self::li(scene, &mut sampler, &its, &ray, false).0,
        }
    }
}
