mod rng;
mod uniform;

use crate::types::*;


pub trait Sampler {
    fn clone_seeded<U>(&self, seed: U) -> Self
        where U: Into<u64>;

    fn gen_1d(&mut self) -> F;
    fn gen_2d(&mut self) -> F2;
}

pub use uniform::Uniform;
