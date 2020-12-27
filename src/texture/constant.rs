use super::*;

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct Constant<A> {
    val: A,
}

impl<A: Copy> Constant<A>
{ #[inline(always)] pub fn eval(&self) -> A { self.val } }

impl<A> Zero for Constant<A> where A: Zero
{ const ZERO: Self = Self { val: A::ZERO }; }
