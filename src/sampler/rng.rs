use rand_core::RngCore;
use rand_pcg::Pcg64;

pub trait RandomFloat<FT> { fn next_f(&mut self) -> FT; }

pub type Prng = Pcg64;

impl RandomFloat<f32> for Prng {
    #[inline] fn next_f(&mut self) -> f32
    { f32::from_bits((self.next_u32() >> 9) | 0x3f80_0000) - 1. }
}

impl RandomFloat<f64> for Prng {
    #[inline] fn next_f(&mut self) -> f64
    { f64::from_bits((self.next_u64() >> 12) | 0x3ff0_0000_0000_0000) - 1. }
}
