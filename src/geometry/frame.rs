use super::*;


#[inline(always)]
pub fn cartesian2spherical(v: F3) -> F2 {
    let y = F::atan2(v[Y], v[X]);
    let y = if y < 0. { y + F::TWO_PI } else { y };
    A2(F::acos(v[Z]), y)
}