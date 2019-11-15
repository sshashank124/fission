use super::*;


#[derive(Clone, Copy)]
pub struct Square {
    radius: F,
}

impl Square {
    #[inline(always)] pub fn new(radius: F) -> Self { Self { radius } }
}

impl Filter for Square {
    #[inline(always)] fn eval(&self, _: F) -> F { 1. }
    #[inline(always)] fn radius(&self) -> F { self.radius }
}
