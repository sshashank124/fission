use std::ops::{Deref, DerefMut};

use super::*;
use super::rng::*;


pub struct Independent(Prng);

impl Independent {
    #[inline(always)] fn from_seed(seed: u64) -> Self
    { Self(Prng::seed_from_u64(seed)) }

    pub fn new() -> Self { Self::from_seed(0) }
}

impl Sample for Independent {
    #[inline(always)]
    fn clone_for_block(&self, (i, Block { pos: A2(x, y), .. }): BlockSeed)
            -> Self {
        Self::from_seed(((i as u64) << 42) +
                        ((*y as u64) << 21) +
                        (*x as u64))
    }

    #[inline(always)] fn prepare_for_pixel(&mut self, _: I2) { }

    #[inline(always)] fn next_1d(&mut self) -> F { self.next_ft() }

    #[inline(always)] fn next_2d(&mut self) -> F2
    { A2(self.next_ft(), self.next_ft()) }

    #[inline(always)] fn rng(&mut self) -> F { self.next_1d() }
}

impl Deref for Independent { type Target = Prng;
    #[inline(always)] fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Independent { #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
