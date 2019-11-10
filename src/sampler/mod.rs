mod rng;
mod uniform;

use crate::types::*;

pub use uniform::Uniform;


pub enum Sampler {
    Uniform(Uniform),
}

pub trait Sample {
    fn clone_seeded<U>(&self, seed: U) -> Self where U: Into<u64>;

    fn gen_1d(&mut self) -> F;
    fn gen_2d(&mut self) -> F2;
}

impl Sample for Sampler {
    fn clone_seeded<U>(&self, seed: U) -> Self where U: Into<u64> {
        match self {
            Sampler::Uniform(s) => Sampler::Uniform(s.clone_seeded(seed))
        }
    }

    fn gen_1d(&mut self) -> F {
        match self { Sampler::Uniform(s) => s.gen_1d() }
    }

    fn gen_2d(&mut self) -> F2 {
        match self { Sampler::Uniform(s) => s.gen_2d() }
    }
}

impl From<Uniform> for Sampler {
    #[inline(always)]
    fn from(s: Uniform) -> Sampler { Sampler::Uniform(s) }
}
