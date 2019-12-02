use super::*;


pub struct Sphere {
    c: P,
    r: F,
}

impl Sphere {
    #[inline(always)] pub fn new(c: P, r: F) -> Self { Self { c, r } }

    #[inline(always)] pub fn intersection_point(&self, ray: R) -> Option<F> {
        let d = ray.o - self.c;
        quad(ray.d.norm2(),
             2. * ray.d.dot(d),
             d.norm2() - self.r.sq())
            .and_then(|A2(t1, t2)| if ray.range().bounds(t1) { Some(t1) }
                                   else if ray.range().bounds(t2) { Some(t2) }
                                   else { None })
    }

    #[inline(always)] fn cartesian2uv(x: F3) -> F2 {
        let uv = cartesian2spherical(x);
        A2(uv[X] * F::INV_PI, 0.5 + uv[Y] * F::INV_2PI)
    }
}

impl Intersectable for Sphere {
    #[inline(always)] fn bbox(&self) -> BBox
    { BBox::ZERO | (self.c - self.r) | (self.c + self.r) }

    #[inline(always)] fn intersects(&self, ray: R) -> bool
    { self.intersection_point(ray).is_some() }

    #[inline(always)] fn intersect(&self, ray: R) -> Option<Its>
    { self.intersection_point(ray)
          .map(|t| Its::new(ray.at(t), N::ZERO, F2::ZERO, t)) }

    #[inline(always)] fn hit_info<'a>(&'a self, mut its: Its<'a>) -> Its<'a> {
        its.n = N::v(its.p - self.c);
        its.uv = Sphere::cartesian2uv(**its.n);
        its
    }

    #[inline(always)] fn sample_surface(&self, s: F2) -> Its {
        let d = V(UniformSphere::warp(s));
        Its::new(self.c + d * self.r, N::v(d), Sphere::cartesian2uv(*d), 0.)
    }

    #[inline(always)] fn surface_area(&self) -> F { F::FOUR_PI * self.r.sq() }

    #[inline(always)] fn intersection_cost(&self) -> F { 2. }
}
