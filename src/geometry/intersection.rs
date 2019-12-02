use std::ops::{Mul, Div};

use super::*;
use crate::bsdf::*;
use crate::light::*;
use crate::shape::*;


pub type ShapeRef<'a> = (&'a Shape, I);
static SHAPE_REF_PH: ShapeRef = (&SHAPE_PH, 0);

#[derive(Clone, Copy)]
pub struct Its<'a> {
    pub p: P,
    pub n: N,
    pub uv: F2,
    pub t: F,
    pub shape: ShapeRef<'a>,
}
pub static ITS_PH: Its = Its { p: P::ZERO, n: N::ZERO, uv: F2::ZERO, t: 0.,
                               shape: SHAPE_REF_PH };

impl<'a> Its<'a> {
    #[inline(always)]
    pub fn its(p: P, n: N, uv: F2, t: F, shape: ShapeRef<'a>) -> Self
    { Self { p, n, uv, t, shape } }

    #[inline(always)] pub fn new(p: P, n: N, uv: F2, t: F) -> Self
    { Self::its(p, n, uv, t, SHAPE_REF_PH) }

    #[inline(always)] pub fn for_shape(mut self, s: &'a Shape) -> Self
    { self.shape = (s, self.shape.1); self }

    #[inline(always)] pub fn for_idx(mut self, idx: usize) -> Self
    { self.shape = (self.shape.0, idx as I); self }

    #[inline(always)] pub fn with_hit_info(self) -> Self
    { self.shape.0.hit_info(self) }

    #[inline(always)] pub fn to_world(&self) -> T { T::from_frame(*self.n) }

    #[inline(always)] pub fn le(&self, ray: &R) -> Color
    { self.shape.0.eval(ray, Some(self.uv)) }

    #[inline(always)] pub fn bsdf(&self) -> &Bsdf { &self.shape.0.bsdf }
}

impl<'a> Mul<Its<'a>> for T { type Output = Its<'a>;
    #[inline(always)] fn mul(self, Its{p, n, uv, t, shape}: Its<'_>) -> Its<'_>
    { Its::its(self * p, self * n, uv, t, shape) }
}

impl<'a> Div<Its<'a>> for T { type Output = Its<'a>;
    #[inline(always)] fn div(self, Its{p, n, uv, t, shape}: Its<'_>) -> Its<'_>
    { Its::its(self / p, self / n, uv, t, shape) }
}
