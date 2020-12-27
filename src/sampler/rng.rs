use rand_core::RngCore;
pub use rand_core::SeedableRng;
use rand_pcg::Pcg64;

pub trait RngFloat<FT> { fn next_f(&mut self) -> FT; }

pub type Prng = Pcg64;

impl RngFloat<f32> for Prng {
    #[inline(always)] fn next_f(&mut self) -> f32
    { f32::from_bits((self.next_u32() >> 9) | 0x3f80_0000) - 1. }
}

impl RngFloat<f64> for Prng {
    #[inline(always)] fn next_f(&mut self) -> f64
    { f64::from_bits((self.next_u64() >> 12) | 0x3ff0_0000_0000_0000) - 1. }
}
