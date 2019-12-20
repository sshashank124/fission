use super::*;

#[derive(Clone, Copy)]
pub struct Norm2 {
    s: F2,
    t: F2,
}

impl One for Norm2 {
    const ONE: Self = Self::new(F2::ONE, F2::ZERO);
}

impl Norm2 {
    const fn new(s: F2, t: F2) -> Self { Self { s, t } }
    pub fn translate(t: F2) -> Self { Self::new(F2::ONE, t) }
    pub fn scale(s: F2) -> Self { Self::new(s, F2::ZERO) }
}

impl Mul for Norm2 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, o: Self) -> Self {
        Self::new(self.s * o.s, self.s * o.t + self.t)
    }
}

impl Mul<F2> for Norm2 {
    type Output = F2;
    #[inline(always)]
    fn mul(self, o: F2) -> F2 { self.s * o + self.t }
}
