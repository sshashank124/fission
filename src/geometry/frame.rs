use super::*;

pub struct Frame;

impl Frame {
    #[inline(always)]
    pub fn ct(v: F3) -> F { v[Z] }
    #[inline(always)]
    pub fn st(v: F3) -> F { F::sqrt(Self::s2t(v)) }
    #[inline(always)]
    pub fn tt(v: F3) -> F { Self::st(v) / Self::ct(v) }

    #[inline(always)]
    pub fn c2t(v: F3) -> F { Self::ct(v).sq() }
    #[inline(always)]
    pub fn s2t(v: F3) -> F { F::clamp_pos(1. - Self::c2t(v)) }
    #[inline(always)]
    pub fn t2t(v: F3) -> F { Self::s2t(v) / Self::c2t(v) }

    #[inline(always)]
    pub fn reflect(v: V) -> V { V(A3(-v[X], -v[Y], v[Z])) }

    // Frame transforms
    #[inline(always)]
    pub fn cart2spher(v: F3) -> F2 {
        let y = F::atan2(v[Y], v[X]);
        let y = if y < 0. { y + F::TWO_PI } else { y };
        A2(F::acos(v[Z]), y)
    }

    #[inline(always)]
    pub fn spher2cart(v: F2) -> F3 {
        let st = F::sin(v[0]);
        A3(st * F::cos(v[1]), st * F::sin(v[1]), F::cos(v[0]))
    }
}
