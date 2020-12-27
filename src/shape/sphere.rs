use super::*;

#[derive(Debug, Deserialize)]
pub struct Sphere {
    #[serde(rename="center")]
    c: P,
    #[serde(rename="radius")]
    r: F,
}

impl Sphere {
    #[inline(always)] pub fn intersection_point(&self, ray: R) -> Option<F> {
        let d = ray.o - self.c;
        quad(ray.d.norm2(),
             2. * F3::dot(ray.d, d),
             d.norm2() - self.r.sq())
            .and_then(|t| if ray.range().bounds(t[0]) { Some(t[0]) }
                          else if ray.range().bounds(t[1]) { Some(t[1]) }
                          else { None })
    }

    #[inline(always)] fn cartesian2uv<A: Into<F3>>(x: A) -> F2 {
        let uv = Frame::cart2spher(x);
        A2(uv[X] * F::INV_PI, 0.5 + uv[Y] * F::INV_2PI)
    }
}

impl Intersectable for Sphere {
    #[inline(always)] fn bbox(&self) -> BBox
    { BBox::ZERO | (self.c - self.r) | (self.c + self.r) }

    #[inline(always)] fn intersects(&self, ray: R) -> bool
    { self.intersection_point(ray).is_some() }

    #[inline(always)] fn intersect(&self, ray: R) -> Option<Its> {
        self.intersection_point(ray)
            .map(|t| Its::new(ray.at(t), N::ZERO, F2::ZERO, t))
    }

    #[inline(always)] fn hit_info<'a>(&'a self, mut its: Its<'a>) -> Its<'a> {
        its.n = N::from(its.p - self.c);
        its.uv = Sphere::cartesian2uv(its.n);
        its
    }

    #[inline(always)] fn sample_surface(&self, s: F2) -> Its {
        let d = V::from(UniformSphere::warp(s));
        Its::new(self.c + d * self.r, N::from(d), Sphere::cartesian2uv(d), 0.)
    }

    #[inline(always)] fn surface_area(&self) -> F { F::FOUR_PI * self.r.sq() }

    fn intersection_cost(&self) -> F { 2. }
}
