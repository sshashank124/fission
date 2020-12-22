use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Constant<A> {
    val: A,
}

impl<A: Zero> Zero for Constant<A> { const ZERO: Self = Self::new(A::ZERO); }

impl<A> Constant<A> { pub const fn new(val: A) -> Self { Self { val } } }

impl<A: Copy> Constant<A>
{ #[inline(always)] pub fn eval(&self) -> A { self.val } }
