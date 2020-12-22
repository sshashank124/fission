use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Checkerboard<A> {
    vals: A2<A>,
    t:    T2,
}

impl<A> Checkerboard<A> {
    pub fn new(val1: A, val2: A, s: Option<F2>, d: Option<F2>) -> Self {
        Self { vals: A2(val2, val1),
               t:    T2::translate(d.unwrap_or(F2::ZERO))
                     * T2::scale(s.unwrap_or(F2::ONE).inv()), }
    }
}

impl<A: Copy> Checkerboard<A> {
    #[inline(always)] pub fn eval(&self, s: F2) -> A {
        self.vals[(self.t * s).map(|f| {
            let r = F::floori(f) % 2;
            if r < 0 { r + 2 } else { r }
        }).reduce(Num::eq)]
    }
}
