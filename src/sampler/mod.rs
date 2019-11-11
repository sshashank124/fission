mod rng;
mod independent;

use crate::types::*;

pub use independent::Independent;


pub enum Sampler {
    Independent(Independent),
}

pub trait Sample {
    fn clone_seeded<U>(&self, seed: U) -> Self where U: Into<u64>;

    fn gen_1d(&mut self) -> F;
    fn gen_2d(&mut self) -> F2;
}

impl Sample for Sampler {
    fn clone_seeded<U>(&self, seed: U) -> Self where U: Into<u64> {
        match self {
            Sampler::Independent(s) =>
                Sampler::Independent(s.clone_seeded(seed))
        }
    }

    fn gen_1d(&mut self) -> F {
        match self { Sampler::Independent(s) => s.gen_1d() }
    }

    fn gen_2d(&mut self) -> F2 {
        match self { Sampler::Independent(s) => s.gen_2d() }
    }
}

impl From<Independent> for Sampler {
    #[inline(always)]
    fn from(s: Independent) -> Sampler { Sampler::Independent(s) }
}
