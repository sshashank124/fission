use std::ops::{Div, Mul};

#[allow(clippy::wildcard_imports)]
use graphite::*;

use crate::bsdf::BSDF;
use crate::color::Color;
use crate::shape::{self, Intersectable, Shape};
use crate::util::pdf::PDF;

pub type ShapeRef<'a> = (&'a Shape, I);
static SHAPE_REF_PH: ShapeRef = (&shape::PLACEHOLDER, 0);

#[derive(Debug)]
pub struct Its<'a> {
    pub p:     P,
    pub n:     N,
    pub uv:    F2,
    pub t:     F,
    pub shape: ShapeRef<'a>,
}

impl<'a> Its<'a> {
    // Constructors
    #[inline] pub const fn its(p: P, n: N, uv: F2, t: F, shape: ShapeRef<'a>) -> Self
    { Self { p, n, uv, t, shape } }

    #[inline] pub fn new(p: P, n: N, uv: F2, t: F) -> Self { Self::its(p, n, uv, t, SHAPE_REF_PH) }

    #[inline] pub const fn for_shape(mut self, s: &'a Shape) -> Self
    { self.shape = (s, self.shape.1); self }

    #[inline] pub fn for_idx(mut self, idx: usize) -> Self
    { self.shape = (self.shape.0, I::of(idx)); self }

    #[inline] pub fn with_hit_info(self) -> Self
    { <&'a Shape>::clone(&self.shape.0).hit_info(self) }

    // Generators
    #[inline] pub fn to_world(&self) -> T { T::from_frame(self.n) }

    #[inline] pub fn spawn_ray(&self, d: V) -> R { R::unbounded(self.p, d) }

    // Queries
    //// Emitter Queries
    #[inline] pub const fn emits(&self) -> bool { self.shape.0.emits() }

    #[inline] pub fn l_emit(&self, ray: R) -> Color
    { if Frame::same_hemisphere(self.n, ray.d) { Color::ZERO }
      else { self.shape.0.eval(self.uv) } }

    #[inline] pub fn l_emit_pdf(&self, ray: R) -> PDF<Color>
    { PDF::new(self.l_emit(ray), self.shape.0.pdf(self, &ray.clipped(self.t))) }

    //// BSDF Queries
    #[inline] pub const fn bsdf(&self) -> &BSDF { &self.shape.0.bsdf }

    #[inline] pub fn bsdf_f(&self, wi: V, wo: V) -> Color { self.bsdf().eval(wi, wo, self.uv) }

    #[inline] pub fn bsdf_f_pdf(&self, wi: V, wo: V) -> PDF<Color>
    { PDF::new(self.bsdf_f(wi, wo), self.bsdf().pdf(wi, wo)) }

    #[inline] pub fn sample_bsdf(&self, wi: V, s: F2) -> (PDF<Color>, V, bool)
    { self.bsdf().sample(wi, self.uv, s) }
}

impl<'a> Mul<Its<'a>> for T {
    type Output = Its<'a>;
    #[inline] fn mul(self, Its { p, n, uv, t, shape }: Its) -> Its
    { Its::its(self * p, self * n, uv, t, shape) }
}

impl<'a> Div<Its<'a>> for T {
    type Output = Its<'a>;
    #[inline] fn div(self, Its { p, n, uv, t, shape }: Its) -> Its
    { Its::its(self / p, self / n, uv, t, shape) }
}
