use super::*;


pub struct Constant<A: Copy> {
    val: A,
}

impl<A: Copy> Constant<A>
{ #[inline(always)] pub fn new(val: A) -> Self { Self { val } } }

impl<A: Copy> Texture<A> for Constant<A>
{ #[inline(always)] fn eval(&self, _: F2) -> A { self.val } }

impl<A: Copy + One> One for Constant<A>
{ const ONE: Self = Self { val: A::ONE }; }
