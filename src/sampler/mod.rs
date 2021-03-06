mod independent;
mod rng;
mod sobol;

#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

use crate::image::rect::Rect;

use independent::Independent;
use sobol::Sobol;

#[derive(Debug, Deserialize)]
#[serde(tag="type", rename_all="snake_case")]
pub enum Sampler {
    Independent(Independent),
    Sobol(Sobol),
}

impl Sampler {
    #[inline] pub fn for_rect(&self, pass: I, rect: &Rect) -> Self {
        match self {
            Self::Independent(_) => Independent::for_rect(pass, rect).into(),
            Self::Sobol(_) => Sobol::for_rect(pass, rect).into(),
        }
    }

    #[inline] pub fn prepare_for_pixel(&mut self, pos: I2) {
        if let Self::Sobol(s) = self {
            s.prepare_for_pixel(pos)
        }
    }

    #[inline] pub fn next_2d(&mut self) -> F2 {
        match self {
            Self::Independent(s) => s.next_2d(),
            Self::Sobol(s) => s.next_2d(),
        }
    }

    #[inline] pub fn rng(&mut self) -> F {
        match self {
            Self::Independent(s) => s.rng(),
            Self::Sobol(s) => s.rng(),
        }
    }
}

#[inline] pub fn split_reuse_2d<A>(s: F2,
                         p: F,
                         f1: impl Fn(F2) -> A,
                         f2: impl Fn(F2) -> A)
                         -> A {
    if s[0] < p { f1(A2(s[0] / p, s[1])) }
    else { f2(A2((s[0] - p) / (1. - p), s[1])) }
}

impl From<Independent> for Sampler
{ #[inline] fn from(s: Independent) -> Self { Self::Independent(s) } }

impl From<Sobol> for Sampler
{ #[inline] fn from(s: Sobol) -> Self { Self::Sobol(s) } }

impl Default for Sampler
{ fn default() -> Self { Self::Independent(Independent::default()) } }
