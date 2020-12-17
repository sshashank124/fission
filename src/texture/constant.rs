use super::*;

#[derive(Debug)]
pub struct Constant<A> {
    val: A,
}

impl<A> Constant<A> {
    pub fn new(val: A) -> Self { Self { val } }
}

impl<A: Copy> Constant<A> {
    #[inline(always)]
    pub fn eval(&self) -> A { self.val }
}

impl<A: Zero> Zero for Constant<A> {
    const ZERO: Self = Self { val: A::ZERO };
}
