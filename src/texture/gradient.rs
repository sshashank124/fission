use std::marker::PhantomData;

use super::*;

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct Gradient<A, L> {
    vals:    A2<A>,
    #[serde(skip)] phantom: PhantomData<L>,
}

impl<A, L> Gradient<A, L>
    where A: Copy + Add<Output = A> + Mul<F, Output = A>,
          L: Interp<A>,
{
    #[inline(always)] pub fn eval(&self, s: F2) -> A
    { L::interp(self.vals, s[0]) }
}
