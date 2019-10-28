use std::ops::{Mul, Div};

use super::*;


#[derive(Clone, Copy, Debug)]
pub struct N(pub V);

impl N {
    #[inline]
    pub fn unit(self) -> N {
        N(self.0.unit())
    }

    #[inline]
    pub fn x(&self) -> F {
        self.0.x()
    }

    #[inline]
    pub fn y(&self) -> F {
        self.0.y()
    }

    #[inline]
    pub fn z(&self) -> F {
        self.0.z()
    }
}

impl Mul<N> for T {
    type Output = N;
    #[inline]
    fn mul(self, N(v): N) -> N {
        N(self.inv().t() * v)
    }
}

impl Div<N> for T {
    type Output = N;
    #[inline]
    fn div(self, N(v): N) -> N {
        N(self.inv().t() / v)
    }
}
