use crate::geometry::*;


#[inline(always)]
pub fn uniform_disk(s: F2) -> F2 {
    let r = s[X].sqrt();
    let t = 2. * F::PI * s[Y];
    P2(r * t.cos(), r * t.sin())
}

#[inline(always)]
pub fn uniform_cylinder(s: F2) -> F3 {
    let t = 2. * F::PI * s[Y];
    A3(t.cos(), t.sin(), 2. * s[X] - 1.)
}

#[inline(always)]
pub fn uniform_sphere(s: F2) -> F3 {
    let v = uniform_cylinder(s);
    let r = (1. - v[Z].sq()).sqrt();
    A3(r * v[X], r * v[Y], v[Z])
}

#[inline(always)]
pub fn uniform_hemisphere(s: F2) -> F3 {
    let v = uniform_sphere(s);
    A3(v[X], v[Y], v[Z].abs())
}
