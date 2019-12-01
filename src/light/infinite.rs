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
    #[inline(always)] fn le(&self, ray: &R) -> Color {
        let uv = cartesian2spherical(A3(ray.d[0], ray.d[2], ray.d[1]));
        self.intensity.eval(uv * A2(F::INV_PI, F::INV_2PI))
    }

    #[inline(always)] fn sample(&self, pos: P, u: F2) -> (Color, R) {
        let sd = V(UniformSphere::warp(u));
        let ray = R::r(pos, sd, F::POS_INF);
        (self.le(&ray), ray)
    }
}
