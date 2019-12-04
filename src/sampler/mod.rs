mod independent;
mod rng;
mod sobol;

use std::ops::{Deref, DerefMut};

use crate::core::*;
use crate::image::Block;

pub use independent::Independent;
pub use sobol::Sobol;


pub type BlockSeed<'a> = (I, &'a Block);  // (sample iteration, pixel)

pub trait RngFloat<FT> { fn next_ft(&mut self) -> FT; }

pub trait Sample {
    fn clone_for_block(&self, seed: BlockSeed) -> Self;
    fn prepare_for_pixel(&mut self, pos: I2);

    fn next_1d(&mut self) -> F;
    fn next_2d(&mut self) -> F2;

    fn rng(&mut self) -> F;
}

pub struct Sampler {
    sampler_type: SamplerType,
    pub spp: I,
}

pub enum SamplerType {
    Independent(Independent),
    Sobol(Sobol),
}

impl Sampler {
    #[inline(always)]
    pub fn new(sampler_type: SamplerType, spp: I) -> Self
    { Self { sampler_type, spp } }

    #[inline(always)]
    pub fn clone_seeded(&self, seed: BlockSeed) -> Self
    { Self::new(self.sampler_type.clone_for_block(seed), self.spp) }

    #[inline(always)]
    pub fn split_reuse_2d<A, FN1, FN2>(s: F2, p: F, f1: FN1, f2: FN2) -> A
        where FN1: Fn(F2) -> A, FN2: Fn(F2) -> A
    { if s[0] < p { f1(A2(s[0] / p, s[1])) }
      else { f2(A2((s[0] - p) / (1. - p), s[1])) } }
}

impl Deref for Sampler { type Target = SamplerType;
    #[inline(always)] fn deref(&self) -> &Self::Target { &self.sampler_type }
}

impl DerefMut for Sampler {
    #[inline(always)] fn deref_mut(&mut self) -> &mut Self::Target
    { &mut self.sampler_type }
}


impl Sample for SamplerType {
    #[inline(always)]
    fn clone_for_block(&self, seed: BlockSeed) -> Self {
        match self {
            Self::Independent(s) => s.clone_for_block(seed).into(),
            Self::Sobol(s) => s.clone_for_block(seed).into(),
        }
    }

    #[inline(always)]
    fn prepare_for_pixel(&mut self, pos: I2) {
        match self {
            Self::Independent(s) => s.prepare_for_pixel(pos),
            Self::Sobol(s) => s.prepare_for_pixel(pos),
        }
    }

    #[inline(always)]
    fn next_1d(&mut self) -> F {
        match self {
            Self::Independent(s) => s.next_1d(),
            Self::Sobol(s) => s.next_1d(),
        }
    }

    #[inline(always)]
    fn next_2d(&mut self) -> F2 {
        match self {
            Self::Independent(s) => s.next_2d(),
            Self::Sobol(s) => s.next_2d(),
        }
    }

    #[inline(always)]
    fn rng(&mut self) -> F {
        match self {
            Self::Independent(s) => s.rng(),
            Self::Sobol(s) => s.rng(),
        }
    }
}

impl From<Independent> for SamplerType
{ #[inline(always)] fn from(s: Independent) -> Self { Self::Independent(s) } }

impl From<Sobol> for SamplerType
{ #[inline(always)] fn from(s: Sobol) -> Self { Self::Sobol(s) } }
