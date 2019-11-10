use crate::geometry::*;
use crate::sampler::*;


#[inline(always)]
pub fn sample_uniform_hemisphere(sampler: &mut Sampler, n: N) -> V {
    let v = loop {
        let v = V::v(1. - 2. * sampler.gen_1d(),
                     1. - 2. * sampler.gen_1d(),
                     1. - 2. * sampler.gen_1d());
        if v.norm2() <= 1. { break v; }
    };

    let v = if v.dot(n.0) < 0. { -v } else { v };
    v.unit()
}


#[inline(always)]
pub fn _square_to_uniform_cylinder(s: F2) -> F3 {
    let phi = 2. * F::PI * s.1;
    A3(phi.cos(), phi.sin(), 2. * s.0 - 1.)
}

#[inline(always)]
pub fn _square_to_uniform_sphere(s: F2) -> F3 {
    let v = _square_to_uniform_cylinder(s);
    let r = (1. - v.2.sq()).sqrt();
    A3(r * v.0, r * v.1, v.2)
}

#[inline(always)]
pub fn _square_to_uniform_hemisphere(s: F2) -> F3 {
    let v = _square_to_uniform_sphere(s);
    A3(v.0, v.1, v.2.abs())
}
