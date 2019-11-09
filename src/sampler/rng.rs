use rand_core::{RngCore, SeedableRng};
use rand_pcg::Pcg64;


pub trait RngFloat<FT> {
    fn gen(&mut self) -> FT;
}


pub struct Rng(Pcg64);

impl Rng {
    #[inline(always)]
    pub fn new() -> Rng {
        Rng(Pcg64::new(0xcafe_f00d_d15e_a5e5,
                       0xa02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96))
    }

    #[inline(always)]
    pub fn from_seed(seed: u64) -> Rng {
        Rng(Pcg64::seed_from_u64(seed))
    }
}

#[repr(C)]
union UF32 {
    u: u32,
    f: f32,
}

impl RngFloat<f32> for Rng {
    #[inline(always)]
    fn gen(&mut self) -> f32 {
        let n = UF32 { u: (self.0.next_u32() >> 9) | 0x3f80_0000 };
        unsafe { n.f - 1. }
    }
}

#[repr(C)]
union UF64 {
    u: u64,
    f: f64,
}

impl RngFloat<f64> for Rng {
    #[inline(always)]
    fn gen(&mut self) -> f64 {
        let n = UF64 { u: (self.0.next_u64() >> 12) | 0x3ff0_0000_0000_0000 };
        unsafe { n.f - 1. }
    }
}
