use super::*;


pub struct Checkerboard<A: Copy> {
    val1: A,
    val2: A,
    t: T2,
}

impl<A: Copy> Checkerboard<A> {
    #[inline(always)]
    pub fn new(val1: A, val2: A, s: Option<F2>, d: Option<F2>) -> Self {
        let s = s.unwrap_or(F2::ONE);
        let d = d.unwrap_or(F2::ZERO);
        Self { val1, val2, t: T2::translate(d) * T2::scale(s.inv()) }
    }
}

impl<A: Copy> Texture<A> for Checkerboard<A> {
    #[inline(always)] fn eval(&self, s: F2) -> A
    { if (self.t * s).map(F::floori).map(I::mod2).reduce(Num::eq)
      { self.val1 } else { self.val2 } }
}
