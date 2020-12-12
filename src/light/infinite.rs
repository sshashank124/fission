use super::*;

pub struct Infinite {
    intensity: Tex<Color>,
}

impl Infinite {
    pub fn new(intensity: Tex<Color>) -> Self { Self { intensity } }

    #[inline(always)]
    pub fn sample(&self, its: &Its, s: F2) -> (Color, R, F) {
        let theta_phi = s * A2(F::PI, F::TWO_PI);
        let sray = R::r(its.p, V(Frame::spher2cart(theta_phi)), F::POS_INF);
        (self.intensity.eval(s), sray, self.pdf(its, &sray))
    }

    #[inline(always)]
    pub fn pdf(&self, its: &Its, sray: &R) -> F {
        F::INV_2PI * F::INV_PI / F::sqrt(1. - its.n.dot(sray.d).sq())
    }

    pub fn is_env_light(&self) -> bool { true }

    #[inline(always)]
    pub fn eval_env(&self, ray: &R) -> Color {
        let uv = Frame::cart2spher(A3(ray.d[0], ray.d[2], ray.d[1]));
        self.intensity.eval(uv * A2(F::INV_PI, F::INV_2PI))
    }
}
