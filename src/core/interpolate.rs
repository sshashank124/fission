use std::ops::{Add, Mul};

use super::*;

pub struct LinearScale;
pub struct PowerScale;
pub struct SmoothScale;

pub trait Interp<A> {
    fn interp(a: A2<A>, t: F) -> A;
}

impl<A> Interp<A> for LinearScale where A: Add<Output = A> + Mul<F, Output = A>
{
    #[inline(always)]
    fn interp(a: A2<A>, t: F) -> A { a.dot(A2(1. - t, t)) }
}

impl<A> Interp<A> for SmoothScale where A: Add<Output = A> + Mul<F, Output = A>
{
    #[inline(always)]
    fn interp(a: A2<A>, t: F) -> A {
        LinearScale::interp(a, t.sq() * (3. - 2. * t))
    }
}

pub trait Balance {
    fn balance(a: F2) -> F;
    #[inline(always)]
    fn balance2(a: F, b: F) -> F { Self::balance(A2(a, b)) }
}

impl Balance for LinearScale {
    #[inline(always)]
    fn balance(a: F2) -> F { a[0] / a.sum() }
}

impl Balance for PowerScale {
    #[inline(always)]
    fn balance(a: F2) -> F { LinearScale::balance(a.map(F::sq)) }
}
