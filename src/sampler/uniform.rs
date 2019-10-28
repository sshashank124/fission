use super::*;
use super::rng::{Rng, RngFloat};


pub struct Uniform(Rng);

impl Uniform {
    #[inline]
    pub fn new() -> Uniform {
        Uniform(Rng::new())
    }
}

impl Sampler for Uniform {
    #[inline]
    fn clone_seeded<U>(&self, seed: U) -> Uniform
            where U: Into<u64> {
        Uniform(Rng::from_seed(seed.into()))
    }

    #[inline]
    fn gen_1d(&mut self) -> F {
        self.0.gen()
    }

    #[inline]
    fn gen_2d(&mut self) -> F2 {
        P2(self.gen_1d(), self.gen_1d())
    }
}
