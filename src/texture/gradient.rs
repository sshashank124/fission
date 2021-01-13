use std::marker::PhantomData;
use std::ops::{Add, Mul};

#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct Gradient<A, L> {
    vals:    A2<A>,
    #[serde(skip)]
    phantom: PhantomData<L>,
}

impl<A, L> Gradient<A, L>
    where A: Copy + Add<Output = A> + Mul<F, Output = A>,
          L: Interp<A>,
{
    #[inline] pub fn eval(&self, s: F2) -> A
    { L::interp(self.vals, s[0]) }

    #[inline] pub fn mean(&self) -> A { self.vals.mean() }
}
