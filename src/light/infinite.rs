use super::*;

use crate::texture::*;


pub struct Infinite {
    intensity: Tex<Color>,
}

impl Infinite
{ pub fn new(intensity: Tex<Color>) -> Self { Self { intensity } } }

impl Lighting for Infinite {
    #[inline(always)] fn eval(&self, _: F2) -> Color { Color::ZERO }

    #[inline(always)] fn sample(&self, its: &Its, s: F2) -> (Color, R, F) {
        let theta_phi = s * A2(F::PI, F::TWO_PI);
        let sray = R::r(its.p, V(spherical2cartesian(theta_phi)), F::POS_INF);
        (self.intensity.eval(s), sray, self.pdf(its, &sray))
    }

    #[inline(always)] fn pdf(&self, its: &Its, sray: &R) -> F
    { F::INV_2PI * F::INV_PI / F::sqrt(1. - its.n.dot(sray.d).sq()) }

    fn is_env_light(&self) -> bool { true }

    #[inline(always)] fn eval_env(&self, ray: &R) -> Color {
        let uv = cartesian2spherical(A3(ray.d[0], ray.d[2], ray.d[1]));
        self.intensity.eval(uv * A2(F::INV_PI, F::INV_2PI))
    }
}
