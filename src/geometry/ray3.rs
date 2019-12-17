use std::ops::{Mul, Div};

use super::*;
use crate::medium::*;
use crate::sampler::*;


#[derive(Clone, Copy)]
pub struct R<'a> {
    pub o: P,
    pub d: V,
    pub t: F,
    pub m: &'a Medium,
}

impl<'a> R<'a> {
    // Constructors
    #[inline(always)] pub fn r_in(o: P, d: V, t: F, m: &'a Medium) -> Self
    { R { o, d, t, m } }

    #[inline(always)] pub fn r(o: P, d: V, t: F) -> Self
    { R::r_in(o, d, t, &Medium::ZERO) }

    #[inline(always)] pub fn unbounded(o: P, d: V) -> Self
    { R::r(o, d.unit(), F::POS_INF) }

    #[inline(always)] pub fn unit(o: P, d: V) -> Self
    { R::r(o, d.unit(), d.norm() - F::EPSILON) }

    #[inline(always)] pub fn p2(a: P, b: P) -> Self { R::unit(a, b - a) }

    // Modifiers
    #[inline(always)] pub fn in_medium(mut self, m: &'a Medium) -> Self
    { self.m = m; self }

    #[inline(always)] pub fn clipped(self, t: F) -> Self
    { R::r_in(self.o, self.d, F::min(t, self.t), self.m) }

    #[inline(always)] pub fn clip(self, its: Option<&Its>) -> Self
    { its.map(|its| self.clipped(its.t)).unwrap_or(self) }

    // Queriers
    #[inline(always)] pub fn at(&self, t: F) -> P { self.o + self.d * t }

    #[inline(always)] pub fn range(&self) -> B { B::b(F::EPSILON, self.t) }

    #[inline(always)] pub fn tr(&self) -> Color { self.m.tr(self) }

    #[inline(always)]
    pub fn sample_tr(&self, sampler: &mut Sampler) -> (Color, Option<Its<'a>>)
    { self.m.sample(self, sampler) }
}

impl<'a> Mul<R<'a>> for T { type Output = R<'a>;
    #[inline(always)] fn mul(self, R{o, d, t, m}: R) -> R
    { R::r_in(self * o, self * d, t, m) }
}

impl<'a> Div<R<'a>> for T { type Output = R<'a>;
    #[inline(always)] fn div(self, R{o, d, t, m}: R) -> R
    { R::r_in(self / o, self / d, t, m) }
}
