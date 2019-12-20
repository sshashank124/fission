use super::*;

pub trait Warp2<Params> {
    fn warp(s: F2, p: Params) -> F2;
    fn pdf(s: F2, p: Params) -> F;
}

pub trait Warp3<Params> {
    fn warp(s: F2, p: Params) -> F3;
    fn pdf(s: F3, p: Params) -> F;
}

pub struct UniformTriangle;
pub struct UniformDisk;
pub struct CosineHemisphere;
pub struct UniformCylinder;
pub struct UniformSphere;
pub struct UniformHemisphere;
pub struct BeckmannHemisphere;

impl Warp2<()> for UniformTriangle {
    #[inline(always)]
    fn warp(s: F2, _: ()) -> F2 {
        let t = s * 0.5;
        let o = t[1] - t[0];
        if F::is_pos(o) {
            A2(t[0], t[1] + o)
        } else {
            A2(t[0] - o, t[1])
        }
    }

    #[inline(always)]
    fn pdf(_: F2, _: ()) -> F { 2. }
}

impl Warp2<()> for UniformDisk {
    #[inline(always)]
    fn warp(s: F2, _: ()) -> F2 {
        let u = s * 2. - 1.;
        if u == F2::ZERO {
            F2::ZERO
        } else {
            let (r, t) = if F::abs(u[X]) > F::abs(u[Y]) {
                (u[X], F::FOURTH_PI * u[Y] / u[X])
            } else {
                (u[Y], F::HALF_PI - F::FOURTH_PI * u[X] / u[Y])
            };
            A2(F::cos(t), F::sin(t)) * r
        }
    }

    #[inline(always)]
    fn pdf(_: F2, _: ()) -> F { F::INV_PI }
}

impl Warp3<()> for CosineHemisphere {
    #[inline(always)]
    fn warp(s: F2, _: ()) -> F3 {
        let p = UniformDisk::warp(s, ());
        A3::a2(p, F::sqrt(1. - p.dot(p)))
    }

    #[inline(always)]
    fn pdf(s: F3, _: ()) -> F { Frame::ct(s) * F::INV_PI }
}

impl Warp3<()> for UniformCylinder {
    #[inline(always)]
    fn warp(s: F2, _: ()) -> F3 {
        let t = F::TWO_PI * s[Y];
        A3(F::cos(t), F::sin(t), 2. * s[X] - 1.)
    }

    #[inline(always)]
    fn pdf(_: F3, _: ()) -> F { F::INV_4PI }
}

impl Warp3<()> for UniformSphere {
    #[inline(always)]
    fn warp(s: F2, _: ()) -> F3 {
        let v = UniformCylinder::warp(s, ());
        let r = Frame::st(v);
        A3(r * v[X], r * v[Y], v[Z])
    }

    #[inline(always)]
    fn pdf(_: F3, _: ()) -> F { F::INV_4PI }
}

impl Warp3<()> for UniformHemisphere {
    #[inline(always)]
    fn warp(s: F2, _: ()) -> F3 {
        let v = UniformSphere::warp(s, ());
        A3(v[X], v[Y], v[Z].abs())
    }

    #[inline(always)]
    fn pdf(_: F3, _: ()) -> F { F::INV_2PI }
}

impl Warp3<F> for BeckmannHemisphere {
    #[inline(always)]
    fn warp(s: F2, alpha: F) -> F3 {
        let c2t = (1. - alpha.sq() * F::ln(F::ONE - s[0])).inv();
        let phi = F::TWO_PI * s[1];
        let r = F::sqrt(1. - c2t);
        A3(r * F::cos(phi), r * F::sin(phi), F::sqrt(c2t))
    }

    #[inline(always)]
    fn pdf(s: F3, alpha: F) -> F {
        let a2_inv = alpha.sq().inv();
        let ct = Frame::ct(s);
        (F::INV_PI * a2_inv * F::exp(-a2_inv * (ct.sq().inv() - 1.)))
        / (ct * ct.sq())
    }
}
