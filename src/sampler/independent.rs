use super::rng::*;
use super::*;

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Independent {
    #[serde(skip)] rng: Prng,
}

impl Independent {
    fn from_seed(seed: u64) -> Self { Self { rng: Prng::seed_from_u64(seed) } }

    #[inline(always)] pub fn for_block(&self, i: I, block: &Block) -> Self {
        Self::from_seed(((i as u64) << 42) + ((block.pos[Y] as u64) << 21)
                                           + (block.pos[X] as u64))
    }

    #[inline(always)] pub fn next_2d(&mut self) -> F2
    { A2(self.rng(), self.rng()) }

    #[inline(always)] pub fn rng(&mut self) -> F { self.rng.next_f() }
}

impl Default for Independent { fn default() -> Self { Self::from_seed(0) } }
