use std::ops::{Mul, Div};

use super::*;


#[derive(Clone, Copy, Debug)]
pub struct R {
    pub o:  P,
    pub d:  V,
    pub tb: B,
}

impl R {
    #[inline(always)]
    pub fn new(o: P, d: V, tb: B) -> R {
        R { o, d, tb }
    }

    #[inline(always)]
    pub fn unbounded(o: P, d: V) -> R {
        R::new(o, d, B::POSITIVE)
    }

    #[inline(always)]
    pub fn clip_max(self, t: F) -> R {
        R::new(self.o, self.d, self.tb.with_upper(t))
    }

    #[inline(always)]
    pub fn clip_from_its(self, its: &Option<Its>) -> R {
        match its {
            None => self,
            Some(its) => self.clip_max(its.t),
        }
    }

    #[inline(always)]
    pub fn at(self, t: F) -> P {
        self.o + t * self.d
    }
}

impl Mul<R> for T {
    type Output = R;
    #[inline(always)]
    fn mul(self, R{o, d, tb}: R) -> R {
        R::new(self * o, self * d, tb)
    }
}

impl Div<R> for T {
    type Output = R;
    #[inline(always)]
    fn div(self, R{o, d, tb}: R) -> R {
        R::new(self / o, self / d, tb)
    }
}
