use std::ops::{Add, Mul};

use super::*;


pub trait Interpolate<A> { fn interp(a: A2<A>, t: F) -> A; }


pub struct LinearInterp;
pub struct SmoothInterp;


impl<A> Interpolate<A> for LinearInterp where A: Add<Output=A>
                                               + Mul<F, Output=A> {
    #[inline(always)] fn interp(a: A2<A>, t: F) -> A
    { a.dot(A2(1. - t, t)) }
}

impl<A> Interpolate<A> for SmoothInterp where A: Add<Output=A>
                                               + Mul<F, Output=A> {
    #[inline(always)] fn interp(a: A2<A>, t: F) -> A
    { LinearInterp::interp(a, t.sq() * (3. - 2. * t)) }
}
