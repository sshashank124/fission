use super::*;

#[derive(Debug, Deserialize)]
pub struct Infinite {
    intensity: Tex<Color>,
}

impl Infinite {
    #[inline] pub fn sample(&self, its: &Its, s: F2) -> (Color, R, F) {
        let theta_phi = s * A2(F::PI, F::TWO_PI);
        let sray = R::unbounded(its.p, V::from(Frame::spher2cart(theta_phi)));
        (self.intensity.eval(s), sray, Self::pdf(its, &sray))
    }

    #[inline] pub fn pdf(its: &Its, sray: &R) -> F
    { F::INV_2PI * F::INV_PI / F::sqrt(1. - F3::dot(its.n, sray.d).sq()) }

    #[inline] pub fn eval_env(&self, ray: &R) -> Color {
        let uv = Frame::cart2spher(F3::from(ray.d).swizzle(0, 2, 1));
        self.intensity.eval(uv * A2(F::INV_PI, F::INV_2PI))
    }
}
