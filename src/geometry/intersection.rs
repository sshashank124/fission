use std::ops::{Mul, Div};

use super::*;
use crate::bsdf::*;
use crate::light::*;
use crate::medium::*;
use crate::shape::*;


pub type ShapeRef<'a> = (&'a Shape, I);
static SHAPE_REF_DUMMY: ShapeRef = (&SHAPE_DUMMY, 0);

pub struct Its<'a> {
    pub p: P,
    pub n: N,
    pub uv: F2,
    pub t: F,
    pub shape: ShapeRef<'a>,
    pub medium: Option<&'a Medium>,
    pub phase: Option<&'a PhaseFn>,
}

impl<'a> Its<'a> {
    // Constructors
    #[inline(always)]
    pub fn its(p: P, n: N, uv: F2, t: F,
               shape: ShapeRef<'a>,
               medium: Option<&'a Medium>,
               phase: Option<&'a PhaseFn>) -> Self
    { Self { p, n, uv, t, shape, medium, phase } }

    #[inline(always)] pub fn new(p: P, n: N, uv: F2, t: F) -> Self
    { Self::its(p, n, uv, t, SHAPE_REF_DUMMY, None, None) }

    #[inline(always)] pub fn local(p: P, n: N, uv: F2) -> Self
    { Self::new(p, n, uv, 0.) }

    // Modifiers
    #[inline(always)] pub fn for_shape(mut self, s: &'a Shape) -> Self
    { self.shape = (s, self.shape.1); self }

    #[inline(always)] pub fn at_idx(mut self, idx: usize) -> Self
    { self.shape = (self.shape.0, idx as I); self }

    #[inline(always)] pub fn in_medium(mut self, medium: &'a Medium) -> Self
    { self.medium = Some(medium); self }

    #[inline(always)] pub fn with_phase(mut self, phase: &'a PhaseFn) -> Self
    { self.phase = Some(phase); self }

    #[inline(always)] pub fn with_hit_info(self) -> Self
    { self.shape.0.hit_info(self) }

    // Ray generators
    #[inline(always)] pub fn ray(&self, d: V) -> R<'a>
    { R::unbounded(self.p, d).in_medium(self.medium_towards(d)) }

    #[inline(always)] pub fn ray_to(&self, p: P) -> R<'a> {
        let ray = R::p2(self.p, p);
        ray.in_medium(self.medium_towards(ray.d))
    }

    #[inline(always)] pub fn ray_for(&self, d: V, t: F) -> R<'a>
    { R::r_in(self.p, d, t, self.medium_towards(d)) }

    #[inline(always)] pub fn medium_towards(&self, d: V) -> &'a Medium {
        if self.phase.is_some() { self.medium.unwrap() } else {
            self.shape().medium_interface.towards(self.n.dot(d))
        }
    }

    // Queriers
    #[inline(always)] pub fn to_world(&self) -> T { T::from_frame(*self.n) }

    #[inline(always)] pub fn shape(&self) -> &'a Shape { &self.shape.0 }

    //// Emitter methods
    #[inline(always)] pub fn le(&self, ray: R) -> Color
    { if self.n.dot(ray.d) >= 0. { Color::BLACK }
      else { self.shape().eval(self.uv) } }

    #[inline(always)] pub fn lpdf(&self, ray: R) -> F
    { self.shape().pdf(self, &ray.clipped(self.t)) }

    #[inline(always)] pub fn has_emission(&self) -> bool
    { self.shape().emission.is_some() }

    //// BSDF methods
    #[inline(always)] pub fn bsdf(&self) -> &BSDF { &self.shape().bsdf }

    #[inline(always)] pub fn lb(&self, wi: V, wo: V) -> Color
    { self.bsdf().eval(wi, wo, self.uv) }

    #[inline(always)] pub fn bpdf(&self, wi: V, wo: V) -> F
    { self.bsdf().pdf(wi, wo) }

    #[inline(always)] pub fn lb_pdf(&self, wi: V, wo: V) -> (Color, F)
    { (self.lb(wi, wo), self.bpdf(wi, wo)) }

    #[inline(always)]
    pub fn sample_lb(&self, wi: V, s: F2) -> (Color, V, F, bool)
    { self.bsdf().sample(wi, self.uv, s) }

    #[inline(always)] pub fn has_bsdf(&self) -> bool { self.bsdf().exists() }
}

impl<'a> Mul<Its<'a>> for T { type Output = Its<'a>;
    #[inline(always)]
    fn mul(self, Its{p, n, uv, t, shape, medium, phase}: Its<'_>) -> Its<'_>
    { Its::its(self * p, self * n, uv, t, shape, medium, phase) }
}

impl<'a> Div<Its<'a>> for T { type Output = Its<'a>;
    #[inline(always)]
    fn div(self, Its{p, n, uv, t, shape, medium, phase}: Its<'_>) -> Its<'_>
    { Its::its(self / p, self / n, uv, t, shape, medium, phase) }
}
