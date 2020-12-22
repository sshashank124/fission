use std::marker::PhantomData;

use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Gradient<A, L> {
    vals:    A2<A>,
    phantom: PhantomData<L>,
}

impl<A, L> Gradient<A, L> {
    pub fn new(val1: A, val2: A) -> Self
    { Self { vals: A2(val1, val2), phantom: PhantomData } }
}

impl<A, L> Gradient<A, L>
    where A: Copy + Add<Output = A> + Mul<F, Output = A>,
          L: Interp<A>
{
    #[inline(always)] pub fn eval(&self, s: F2) -> A
    { L::interp(self.vals, s[0]) }
}
