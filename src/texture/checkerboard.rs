use std::ops::{Add, Mul};

#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(from="CheckerboardConfig<A>")]
pub struct Checkerboard<A> {
    vals: A2<A>,
    t:    T2,
}

impl<A: Copy> Checkerboard<A> {
    #[inline] pub fn eval(&self, s: F2) -> A {
        self.vals[(self.t * s).map(|f| {
            let r = F::floori(f) % 2;
            if r < 0 { r + 2 } else { r }
        }).as_ref().reduce(I::eq)]
    }
}

impl<A> Checkerboard<A> where A: Copy + Zero + Add<Output=A> + Mul<F, Output=A> {
    #[inline] pub fn mean(&self) -> A { self.vals.mean() }
}


#[derive(Debug, Deserialize)]
struct CheckerboardConfig<A> {
    vals:  A2<A>,
    scale: Option<F2>,
    delta: Option<F2>,
}

impl<A> From<CheckerboardConfig<A>> for Checkerboard<A> {
    fn from(cc: CheckerboardConfig<A>) -> Self {
        Self {
            vals: cc.vals,
            t: T2::translate(cc.delta.unwrap_or(A2::ZERO))
             * T2::scale(cc.scale.unwrap_or(A2::ONE).inv()),
        }
    }
}
