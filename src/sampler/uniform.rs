use super::*;
use super::rng::{Rng, RngFloat};


pub struct Uniform(Rng);

impl Uniform {
    #[inline(always)]
    pub fn new() -> Uniform {
        Uniform(Rng::new())
    }
}

impl Sample for Uniform {
    #[inline(always)]
    fn clone_seeded<U>(&self, seed: U) -> Uniform
            where U: Into<u64> {
        Uniform(Rng::from_seed(seed.into()))
    }

    #[inline(always)]
    fn gen_1d(&mut self) -> F {
        self.0.gen()
    }

    #[inline(always)]
    fn gen_2d(&mut self) -> F2 {
        P2(self.gen_1d(), self.gen_1d())
    }
}
