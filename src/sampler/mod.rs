mod independent;
mod rng;
mod sobol;

use std::ops::{Deref, DerefMut};

use crate::prelude::*;
use crate::image::Block;

pub use independent::Independent;
pub use sobol::Sobol;

pub struct Sampler {
    sampler: SamplerType,
    pub spp: I,
}

pub enum SamplerType {
    Independent(Independent),
    Sobol(Sobol),
}

impl Sampler {
    pub fn new(sampler: SamplerType, spp: I) -> Self { Self { sampler, spp } }

    #[inline(always)]
    pub fn for_block(&self, i: I, block: &Block) -> Self {
        Self::new(self.sampler.for_block(i, block), self.spp)
    }

    #[inline(always)]
    pub fn prepare_for_pixel(&mut self, pos: I2) {
        self.sampler.prepare_for_pixel(pos)
    }

    #[inline(always)]
    pub fn split_reuse_2d<A>(s: F2,
                             p: F,
                             f1: impl Fn(F2) -> A,
                             f2: impl Fn(F2) -> A)
                             -> A {
        if s[0] < p {
            f1(A2(s[0] / p, s[1]))
        } else {
            f2(A2((s[0] - p) / (1. - p), s[1]))
        }
    }
}

impl Deref for Sampler {
    type Target = SamplerType;
    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.sampler }
}

impl DerefMut for Sampler {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.sampler }
}

impl SamplerType {
    #[inline(always)]
    pub fn for_block(&self, i: I, block: &Block) -> Self {
        match self {
            Self::Independent(s) => s.for_block(i, block).into(),
            Self::Sobol(s) => s.for_block(i, block).into(),
        }
    }

    #[inline(always)]
    pub fn prepare_for_pixel(&mut self, pos: I2) {
        match self {
            Self::Sobol(s) => s.prepare_for_pixel(pos),
            _ => (),
        }
    }

    #[inline(always)]
    pub fn next_2d(&mut self) -> F2 {
        match self {
            Self::Independent(s) => s.next_2d(),
            Self::Sobol(s) => s.next_2d(),
        }
    }

    #[inline(always)]
    pub fn rng(&mut self) -> F {
        match self {
            Self::Independent(s) => s.rng(),
            Self::Sobol(s) => s.rng(),
        }
    }
}

impl From<Independent> for SamplerType {
    fn from(s: Independent) -> Self { Self::Independent(s) }
}

impl From<Sobol> for SamplerType {
    fn from(s: Sobol) -> Self { Self::Sobol(s) }
}
