use super::*;


pub trait Warp2: Pdf<F2> { fn warp(s: F2) -> F2; }
pub trait Warp3: Pdf<F3> { fn warp(s: F2) -> F3; }


pub struct UniformDisk;
pub struct CosineHemisphere;
pub struct UniformCylinder;
pub struct UniformSphere;
pub struct UniformHemisphere;


impl Warp2 for UniformDisk {
    #[inline(always)] fn warp(s: F2) -> F2 {
        let u = s * 2. - 1.;
        if u == F2::ZERO { F2::ZERO }
        else {
            let (r, t) = if F::abs(u[X]) > F::abs(u[Y])
                { (u[X], F::FOURTH_PI * u[Y] / u[X]) }
                else { (u[Y], F::HALF_PI - F::FOURTH_PI * u[X] / u[Y]) };
            A2(F::cos(t), F::sin(t)) * r
        }
    }
}

impl Pdf<F2> for UniformDisk
{ #[inline(always)] fn pdf(_: F2) -> F { F::INV_PI } }


impl Warp3 for CosineHemisphere {
    #[inline(always)] fn warp(s: F2) -> F3 {
        let p = UniformDisk::warp(s);
        A3::a2(p, F::sqrt(1. - p.dot(p)))
    }
}

impl Pdf<F3> for CosineHemisphere
{ #[inline(always)] fn pdf(s: F3) -> F { s[Z] * F::INV_PI } }


impl Warp3 for UniformCylinder {
    #[inline(always)] fn warp(s: F2) -> F3 {
        let t = F::TWO_PI * s[Y];
        A3(F::cos(t), F::sin(t), 2. * s[X] - 1.)
    }
}

impl Pdf<F3> for UniformCylinder
{ #[inline(always)] fn pdf(_: F3) -> F { F::INV_4PI } }


impl Warp3 for UniformSphere {
    #[inline(always)] fn warp(s: F2) -> F3 {
        let v = UniformCylinder::warp(s);
        let r = F::sqrt(1. - v[Z].sq());
        A3(r * v[X], r * v[Y], v[Z])
    }
}

impl Pdf<F3> for UniformSphere
{ #[inline(always)] fn pdf(_: F3) -> F { F::INV_4PI } }


impl Warp3 for UniformHemisphere {
    #[inline(always)] fn warp(s: F2) -> F3 {
        let v = UniformSphere::warp(s);
        A3(v[X], v[Y], v[Z].abs())
    }
}

impl Pdf<F3> for UniformHemisphere
{ #[inline(always)] fn pdf(_: F3) -> F { F::INV_2PI } }
