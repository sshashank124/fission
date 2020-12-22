use rand_core::RngCore;
pub use rand_core::SeedableRng;
use rand_pcg::Pcg64;

pub trait RngFloat<FT> { fn next_f(&mut self) -> FT; }

pub type Prng = Pcg64;

impl RngFloat<f32> for Prng {
    #[inline(always)] fn next_f(&mut self) -> f32
    { u32_to_f32(self.next_u32()) }
}

impl RngFloat<f64> for Prng {
    #[inline(always)] fn next_f(&mut self) -> f64
    { u64_to_f64(self.next_u64()) }
}

#[inline(always)] fn u32_to_f32(i: u32) -> f32 {
    let u = (i >> 9) | 0x3f80_0000;
    let f = unsafe { std::mem::transmute::<u32, f32>(u) };
    f - 1.
}

#[inline(always)] fn u64_to_f64(i: u64) -> f64 {
    let u = (i >> 12) | 0x3ff0_0000_0000_0000;
    let f = unsafe { std::mem::transmute::<u64, f64>(u) };
    f - 1.
}
