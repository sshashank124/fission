use std::ops::BitAnd;

use super::*;

pub struct Grid<A> {
    vals:      A2<A>,
    normalize: T2,
    fill:      I,
    padding:   F,
}

impl<A> Grid<A> {
    pub fn new(val1: A,
               val2: A,
               scale: Option<F2>,
               delta: Option<F2>,
               fill: Option<F>,
               padding: Option<F>)
               -> Self {
        Self { vals:      A2(val2, val1),
               normalize: T2::translate(delta.unwrap_or(F2::ZERO))
                          * T2::scale(scale.unwrap_or(F2::ONE).inv()),
               fill:      F::floori(fill.unwrap_or(1.) * 100.),
               padding:   0.5 - padding.unwrap_or(0.), }
    }
}

impl<A: Copy> Texture<A> for Grid<A> {
    #[inline(always)]
    fn eval(&self, s: F2) -> A {
        let cell = self.normalize * s;
        let ci = cell.map(F::floori);
        let p = shuffle_u64(((ci[X] as u64) << 20) + ci[Y] as u64);
        if (p % 100) >= self.fill as u64 {
            self.vals[0]
        } else {
            self.vals[(cell - F2::from(ci) - 0.5).map(|f| {
                                                     F::abs(f) < self.padding
                                                 })
                                                 .reduce(BitAnd::bitand)]
        }
    }
}
