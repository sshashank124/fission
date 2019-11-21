use crate::types::*;


pub type Prob<A> = (A, F);  // corresponding pdf

#[inline(always)]
pub fn uniform_disk(s: F2) -> Prob<F2> {
    let u = s * 2. - 1.;
    (if u == F2::ZERO { F2::ZERO }
    else {
        let (r, t) = if F::abs(u[X]) > F::abs(u[Y])
        { (u[X], F::FOURTH_PI * u[Y] / u[X]) }
        else { (u[Y], F::HALF_PI - F::FOURTH_PI * u[X] / u[Y]) };
        A2(F::cos(t), F::sin(t)) * r
    }, F::INV_PI)
}

#[inline(always)]
pub fn cosine_hemisphere(s: F2) -> Prob<F3> {
    let p = uniform_disk(s).0;
    let z = F::sqrt(1. - p.dot(p));
    (A3::a2(p, z), z * F::INV_PI)
}

#[inline(always)]
pub fn uniform_cylinder(s: F2) -> F3 {
    let t = F::TWO_PI * s[Y];
    A3(t.cos(), t.sin(), 2. * s[X] - 1.)
}

#[inline(always)]
pub fn uniform_sphere(s: F2) -> Prob<F3> {
    let v = uniform_cylinder(s);
    let r = F::sqrt(1. - v[Z].sq());
    (A3(r * v[X], r * v[Y], v[Z]), F::INV_4PI)
}

#[inline(always)]
pub fn uniform_hemisphere(s: F2) -> Prob<F3> {
    let v = uniform_sphere(s).0;
    (A3(v[X], v[Y], v[Z].abs()), F::INV_2PI)
}
