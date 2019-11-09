use std::ops::{Mul, Div};

use super::*;


#[derive(Clone, Copy, Debug)]
pub struct N(pub V);

impl N {
    #[inline(always)]
    pub fn unit(self) -> N {
        N(self.0.unit())
    }

    #[inline(always)]
    pub fn x(&self) -> F {
        self.0.x()
    }

    #[inline(always)]
    pub fn y(&self) -> F {
        self.0.y()
    }

    #[inline(always)]
    pub fn z(&self) -> F {
        self.0.z()
    }
}

impl Mul<N> for T {
    type Output = N;
    #[inline(always)]
    fn mul(self, N(v): N) -> N {
        N(self.inv().t() * v)
    }
}

impl Div<N> for T {
    type Output = N;
    #[inline(always)]
    fn div(self, N(v): N) -> N {
        N(self.inv().t() / v)
    }
}
