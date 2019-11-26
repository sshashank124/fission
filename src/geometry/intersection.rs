use std::ops::{Mul, Div};

use super::*;
use crate::shape::*;


pub type ShapeRef<'a> = (Option<&'a Shape>, I);

pub struct Its<'a> {
    pub p: P,
    pub n: N,
    pub uv: F2,
    pub t: F,
    pub shape: ShapeRef<'a>,
}

impl<'a> Its<'a> {
    #[inline(always)]
    pub fn its(p: P, n: N, uv: F2, t: F, shape: ShapeRef<'a>) -> Self
    { Self { p, n, uv, t, shape } }

    #[inline(always)] pub fn new(p: P, n: N, uv: F2, t: F) -> Self
    { Self::its(p, n, uv, t, (None, 0)) }

    #[inline(always)] pub fn for_shape(mut self, s: &'a Shape) -> Self {
        match self.shape.0 {
            None => { self.shape = (Some(s), self.shape.1); self },
            Some(_) => self,
        }
    }

    #[inline(always)]
    pub fn for_idx(mut self, idx: I) -> Self {
        match self.shape.0 {
            None => { self.shape = (self.shape.0, idx); self },
            Some(_) => self,
        }
    }

    #[inline(always)] pub fn with_hit_info(self) -> Self {
        match self.shape.0 {
            None => self,
            Some(s) => s.hit_info(self),
        }
    }
}

impl<'a> Mul<Its<'a>> for T { type Output = Its<'a>;
    #[inline(always)] fn mul(self, Its{p, n, uv, t, shape}: Its<'_>) -> Its<'_>
    { Its::its(self * p, self * n, uv, t, shape) }
}

impl<'a> Div<Its<'a>> for T { type Output = Its<'a>;
    #[inline(always)] fn div(self, Its{p, n, uv, t, shape}: Its<'_>) -> Its<'_>
    { Its::its(self / p, self / n, uv, t, shape) }
}
