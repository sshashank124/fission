use std::ops::{Add, Mul};

use super::*;


#[derive(Clone, Copy, Debug)]
pub struct RotScale3(A3<F3>);

impl RotScale3 {
    #[inline(always)]
    pub fn t(self) -> RotScale3 {
        RotScale3(self.0.t())
    }

    pub const I: RotScale3 = RotScale3(A3(F3::X, F3::Y, F3::Z));
}

impl Mul for RotScale3 {
    type Output = RotScale3;
    #[inline(always)]
    fn mul(self, m: RotScale3) -> RotScale3 {
        RotScale3(self * m.0)
    }
}

impl<B, Z> Mul<A3<B>> for RotScale3 where B: Copy + Mul<F, Output=Z>,
                                          Z: Add<Z, Output=Z> {
    type Output = A3<Z>;
    #[inline(always)]
    fn mul(self, o: A3<B>) -> A3<Z> {
        zip(rep(o), self.0, dot)
    }
}
