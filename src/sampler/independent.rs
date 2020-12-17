use std::ops::{Deref, DerefMut};

use super::rng::*;
use super::*;

#[derive(Clone)]
pub struct Independent(Prng);

impl Independent {
    #[inline(always)]
    fn from_seed(seed: u64) -> Self { Self(Prng::seed_from_u64(seed)) }

    pub fn new() -> Self { Self::from_seed(0) }

    #[inline(always)]
    pub fn for_block(&self, i: I, Block { pos: A2(x, y), .. }: &Block) -> Self {
        Self::from_seed(((i as u64) << 42) + ((*y as u64) << 21) + (*x as u64))
    }

    #[inline(always)]
    pub fn next_2d(&mut self) -> F2 { A2(self.next_f(), self.next_f()) }

    #[inline(always)]
    pub fn rng(&mut self) -> F { self.next_f() }
}

impl Deref for Independent {
    type Target = Prng;
    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Independent {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
