use std::ops::{Deref, DerefMut};

use super::*;
use super::rng::{Rng, RngFloat};


pub struct Independent(Rng);

impl Independent {
    #[inline(always)]
    pub fn new() -> Self {
        Self(Rng::new())
    }
}

impl Sample for Independent {
    #[inline(always)]
    fn clone_seeded<U>(&self, seed: U) -> Self
            where U: Into<u64> {
        Self(Rng::from_seed(seed.into()))
    }

    #[inline(always)]
    fn gen_1d(&mut self) -> F {
        self.gen()
    }

    #[inline(always)]
    fn gen_2d(&mut self) -> F2 {
        P2(self.gen_1d(), self.gen_1d())
    }
}

impl Deref for Independent {
    type Target = Rng;
    #[inline(always)]
    fn deref(&self) -> &Rng { &self.0 }
}

impl DerefMut for Independent {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Rng { &mut self.0 }
}
