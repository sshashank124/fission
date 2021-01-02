use super::*;

pub struct Direct;

impl Direct {
    #[inline(always)] pub fn li<'a>(scene: &'a Scene, sampler: &mut Sampler,
                                    its: &Its, ray: &R)
            -> (Color, Option<(Color, R, Its<'a>, bool)>) {
        let frame = its.to_world();
        let wo = frame / -ray.d;

        let ll = Self::ll(scene, sampler, its, frame, wo);
        let (ls, details) = Self::ls(scene, sampler, its, frame, wo);

        (ll + ls, details)
    }

    #[inline(always)] pub fn ll(scene: &Scene, sampler: &mut Sampler, its: &Its,
                                frame: T, wo: V) -> Color {
        if !its.bsdf().is_delta() {
            let (le, sray, lpdf) = scene.sample_random_light(its,
                                                             sampler.next_2d());
            if le != Color::ZERO && !scene.intersects(sray) {
                let wi = frame / sray.d;
                let lb = its.lb(wo, wi);
                if lb != Color::ZERO {
                    return le
                           * lb
                           * scene.lights_dpdf.total()
                           * PowerScale::balance2(lpdf, its.bpdf(wo, wi))
                }
            }
        }
        Color::ZERO
    }

    #[inline(always)] pub fn ls<'a>(scene: &'a Scene, sampler: &mut Sampler,
                                    its: &Its, frame: T, wo: V)
            -> (Color, Option<(Color, R, Its<'a>, bool)>) {
        let (lb, wi, b_pdf, spec) = its.sample_lb(wo, sampler.next_2d());
        if lb == Color::ZERO { return (Color::ZERO, None) }

        let ray = its.spawn_ray(frame * wi);
        let its = scene.intersect(ray);

        let (le, l_pdf) = its.as_ref().map_or((Color::ZERO, 0.), |its|
            if its.has_emission() { (its.le(ray), its.lpdf(ray)) }
            else { (Color::ZERO, 0.) }
        );

        let ls = if l_pdf > 0. && le != Color::ZERO && !spec {
            lb * le * PowerScale::balance2(b_pdf, l_pdf)
        } else { Color::ZERO };

        (ls, its.map(|i| (lb, ray, i, spec)))
    }

    #[inline(always)]
    pub fn trace(scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
        match scene.intersect(ray) {
            None => scene.lenv(&ray),
            Some(its) => {
                its.le(ray) + Self::li(scene, sampler, &its, &ray).0
            }
        }
    }
}
