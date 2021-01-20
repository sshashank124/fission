#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

use crate::shape::{Intersectable, Its};

#[derive(Debug, Deserialize)]
pub struct Sphere {
    center: P,
    radius: F,
}

impl Sphere {
    #[inline] pub fn intersection_point(&self, ray: R) -> Option<F> {
        let d = ray.o - self.center;
        quad(ray.d.norm2(),
             2. * F3::dot(ray.d.conv(), d.conv()),
             d.norm2() - self.radius.sq())
            .and_then(|t| if ray.range().bounds(t[0]) { Some(t[0]) }
                          else if ray.range().bounds(t[1]) { Some(t[1]) }
                          else { None })
    }

    #[inline] fn cartesian2uv<A: Conv<F3>>(x: A) -> F2 {
        let uv = Frame::cart2spher(x);
        A2(uv[X] * F::INV_PI, uv[Y].mul_add(F::INV_2PI, 0.5))
    }
}

impl Intersectable for Sphere {
    #[inline] fn bbox(&self) -> BBox
    { BBox::ZERO | (self.center - self.radius) | (self.center + self.radius) }

    #[inline] fn intersects(&self, ray: R) -> bool { self.intersection_point(ray).is_some() }

    #[inline] fn intersect(&self, ray: R) -> Option<Its> {
        self.intersection_point(ray)
            .map(|t| Its::new(ray.at(t), N::ZERO, F2::ZERO, t))
    }

    #[inline] fn hit_info<'a>(&'a self, mut its: Its<'a>) -> Its<'a> {
        its.n = (its.p - self.center).conv();
        its.uv = Self::cartesian2uv(its.n);
        its
    }

    #[inline] fn sample_surface(&self, s: F2) -> Its {
        let d: V = UniformSphere::warp(s).conv();
        Its::new(self.center + d * self.radius, d.conv(), Self::cartesian2uv(d), 0.)
    }

    #[inline] fn surface_area(&self) -> F { F::FOUR_PI * self.radius.sq() }

    fn intersection_cost(&self) -> F { 2. }
}
