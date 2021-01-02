use super::rng::*;
use super::*;

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Independent {
    #[serde(skip)] rng: Prng,
}

impl Independent {
    fn from_seed(seed: u64) -> Self { Self { rng: Prng::seed_from_u64(seed) } }

    #[inline] pub fn for_rect(pass: I, rect: &Rect) -> Self {
        Self::from_seed(((pass as u64) << 42) + ((rect.pos[Y] as u64) << 21)
                                           + (rect.pos[X] as u64))
    }

    #[inline] pub fn next_2d(&mut self) -> F2
    { A2(self.rng(), self.rng()) }

    #[inline] pub fn rng(&mut self) -> F { self.rng.next_f() }
}

impl Default for Independent { fn default() -> Self { Self::from_seed(0) } }
