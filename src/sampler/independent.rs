#[allow(clippy::wildcard_imports)]
use graphite::*;
use rand_core::SeedableRng;
use serde::Deserialize;

use crate::image::rect::Rect;

use super::rng::{Prng, RandomFloat};

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Independent {
    #[serde(skip)] rng: Prng,
}

impl Independent {
    fn from_seed(seed: u64) -> Self { Self { rng: Prng::seed_from_u64(seed) } }

    #[inline] pub fn for_rect(pass: I, rect: &Rect) -> Self {
        Self::from_seed((u64::of(pass) << 42) + (u64::of(rect.pos[Y]) << 21)
                                              + u64::of(rect.pos[X]))
    }

    #[inline] pub fn next_2d(&mut self) -> F2
    { A2(self.rng(), self.rng()) }

    #[inline] pub fn rng(&mut self) -> F { self.rng.next_f() }
}

impl Default for Independent { fn default() -> Self { Self::from_seed(0) } }
