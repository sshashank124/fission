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
        its.uv = cartesian2spherical(**its.n);
        its.uv[X] = 0.5 + its.uv[X] * F::INV_2PI;
        its.uv[Y] *= F::INV_PI;
        its
    }

    #[inline(always)] fn intersection_cost(&self) -> F { 2. }
}
