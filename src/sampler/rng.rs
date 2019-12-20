pub use rand_core::SeedableRng;
use rand_core::RngCore;
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

#[repr(C)] union UF32 { u: u32, f: f32 }
#[inline(always)] pub fn u32_to_f32(i: u32) -> f32
{ let n = UF32 { u: (i >> 9) | 0x3f80_0000 }; unsafe { n.f - 1. } }

#[repr(C)] union UF64 { u: u64, f: f64 }
#[inline(always)] pub fn u64_to_f64(i: u64) -> f64
{ let n = UF64 { u: (i >> 12) | 0x3ff0_0000_0000_0000 }; unsafe { n.f - 1. } }
