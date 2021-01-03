use std::ops::{Div, Mul};

#[allow(clippy::wildcard_imports)]
use graphite::*;

use crate::bsdf::BSDF;
use crate::shape::{self, Intersectable, Shape};

pub type ShapeRef<'a> = (&'a Shape, I);
static SHAPE_REF_PH: ShapeRef = (&shape::PLACEHOLDER, 0);

pub struct Its<'a> {
    pub p:     P,
    pub n:     N,
    pub uv:    F2,
    pub t:     F,
    pub shape: ShapeRef<'a>,
}

impl<'a> Its<'a> {
    #[inline]
    pub const fn its(p: P, n: N, uv: F2, t: F, shape: ShapeRef<'a>) -> Self
    { Self { p, n, uv, t, shape } }

    #[inline] pub fn new(p: P, n: N, uv: F2, t: F) -> Self
    { Self::its(p, n, uv, t, SHAPE_REF_PH) }

    #[inline] pub const fn for_shape(mut self, s: &'a Shape) -> Self
    { self.shape = (s, self.shape.1); self }

    #[inline] pub const fn for_idx(mut self, idx: usize) -> Self
    { self.shape = (self.shape.0, idx as I); self }

    #[inline] pub fn with_hit_info(self) -> Self
    { self.shape.0.hit_info(self) }

    #[inline] pub fn to_world(&self) -> T { T::from_frame(self.n) }

    #[inline] pub fn spawn_ray(&self, d: V) -> R
    { R::unbounded(self.p, d) }

    #[inline] pub const fn shape(&self) -> &Shape { self.shape.0 }

    #[inline] pub fn le(&self, ray: R) -> Color
    { if F3::dot(self.n, ray.d) >= 0. { Color::ZERO }
      else { self.shape().eval(self.uv) } }

    #[inline] pub fn lpdf(&self, ray: R) -> F
    { self.shape().pdf(self, &ray.clipped(self.t)) }

    #[inline] pub const fn has_emission(&self) -> bool
    { self.shape().emission.is_some() }

    #[inline] pub const fn bsdf(&self) -> &BSDF { &self.shape().bsdf }

    #[inline] pub fn lb(&self, wi: V, wo: V) -> Color
    { self.bsdf().eval(wi, wo, self.uv) }

    #[inline] pub fn bpdf(&self, wi: V, wo: V) -> F
    { self.bsdf().pdf(wi, wo) }

    #[inline]
    pub fn sample_lb(&self, wi: V, s: F2) -> (Color, V, F, bool)
    { self.bsdf().sample(wi, self.uv, s) }
}

impl<'a> Mul<Its<'a>> for T {
    type Output = Its<'a>;
    #[inline]
    fn mul(self, Its { p, n, uv, t, shape }: Its<'_>) -> Its<'_>
    { Its::its(self * p, self * n, uv, t, shape) }
}

impl<'a> Div<Its<'a>> for T {
    type Output = Its<'a>;
    #[inline]
    fn div(self, Its { p, n, uv, t, shape }: Its<'_>) -> Its<'_>
    { Its::its(self / p, self / n, uv, t, shape) }
}
