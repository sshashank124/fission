use super::*;

pub struct Constant<A> {
    val: A,
}

impl<A> Constant<A> {
    pub fn new(val: A) -> Self { Self { val } }
}

impl<A: Copy> Texture<A> for Constant<A> {
    #[inline(always)]
    fn eval(&self, _: F2) -> A { self.val }
}

impl<A: Zero> Zero for Constant<A> {
    const ZERO: Self = Self { val: A::ZERO };
}
