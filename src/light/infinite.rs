use super::*;

use crate::texture::*;


pub struct Infinite {
    intensity: Tex<Color>,
}

impl Infinite {
    #[inline(always)] pub fn new(intensity: Tex<Color>) -> Self
    { Self { intensity } }
}

impl Lighting for Infinite {
    #[inline(always)] fn eval(&self, ray: &R, _: Option<F2>) -> Color {
        let uv = cartesian2spherical(A3(ray.d[0], ray.d[2], ray.d[1]));
        self.intensity.eval(uv * A2(F::INV_PI, F::INV_2PI))
    }

    #[inline(always)] fn sample(&self, its: &Its, s: F2) -> (Color, R) {
        let theta_phi = s * A2(F::PI, F::TWO_PI);
        let ray = R::r(its.p, V(spherical2cartesian(theta_phi)), F::POS_INF);
        (self.intensity.eval(s), ray)
    }

    #[inline(always)] fn pdf(&self, its: &Its, sray: &R) -> F
    { F::INV_2PI * F::INV_PI / F::sqrt(1. - its.n.dot(sray.d).sq()) }
}
