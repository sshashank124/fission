mod sobol;
mod vdc;
mod vdc_inv;

use super::*;
use sobol::*;
use vdc::*;
use vdc_inv::*;


pub struct Sobol {
    dim: u32,
    m: u32,
    cache: SampleIndexMemo,
    block_pos: I2,
    pixel_pos: I2,
    rng: Independent,
}

struct SampleIndexMemo {
    d: u64,
    i: u64,
    vdc_inv: &'static [u64],
}

impl Sobol {
    #[inline(always)] pub fn new() -> Self {
        Self {
            dim: 0, m: 0, cache: SampleIndexMemo::default(),
            block_pos: I2::ZERO, pixel_pos: I2::ZERO,
            rng: Independent::new(),
        }
    }

    #[inline(always)]
    fn sample_index(&self) -> u64 {
        if self.m == 0 { return 0; }
        let P2(x, y) = self.pixel_pos - self.block_pos;
        let mut d = self.cache.d ^ (y as u64) | ((x as u64) << self.m);
        let mut i = self.cache.i;
        let mut c = 0;
        while d != 0 {
            if (d & 1) == 1 { i ^= self.cache.vdc_inv[c]; }
            d >>= 1; c += 1;
        }
        i
    }
}

impl Sample for Sobol {
    #[inline(always)]
    fn clone_for_block(&self, block_seed: BlockSeed) -> Self {
        let (i, Block { pos, dims, .. }) = block_seed;
        let res = ceil_pow2_u32(Num::max(dims[X], dims[Y]) as u32);
        let m = log2_ceil_u32(res);
        let cache = SampleIndexMemo::new(i as u64, m);
        Self {
            dim: 0, m, cache,
            block_pos: *pos, pixel_pos: I2::ZERO,
            rng: self.rng.clone_for_block(block_seed),
        }
    }

    #[inline(always)]
    fn prepare_pixel(&mut self, pixel: &Pixel) {
        self.dim = 0;
        self.pixel_pos = pixel.pos;
        self.rng.prepare_pixel(pixel);
    }

    #[inline(always)]
    fn next_1d(&mut self) -> F {
        if self.dim >= SOBOL_NDIM {
            eprintln!("Sobol Sampler: dimension overflow, using rng");
            return self.rng();
        }

        let mut f = sample_float(self.sample_index(), self.dim);

        if self.dim < 2 {
            f = (f * (1 << self.m) as F) + self.block_pos[self.dim as I] as F;
            f = Num::clamp_unit(f - self.pixel_pos[self.dim as I] as F);
        }

        self.dim += 1;
        f
    }

    #[inline(always)]
    fn next_2d(&mut self) -> F2 { P2(self.next_1d(), self.next_1d()) }

    #[inline(always)] fn rng(&mut self) -> F { self.rng.rng() }
}

#[inline(always)]
fn sample_float(idx: u64, dim: u32) -> F {
    sample_sobol(idx, dim) as F * F::FRAC_1_2POW32
}

#[inline(always)]
fn sample_sobol(mut idx: u64, dim: u32) -> u32 {
    let mut res = 0; let mut loc = dim * SOBOL_SIZE;
    while idx != 0 {
        if (idx & 1) == 1 { res ^= SOBOL_MATRIX[loc as usize]; }
        idx >>= 1; loc += 1;
    }
    res
}

impl SampleIndexMemo {
    #[inline(always)]
    fn default() -> Self { Self { d: 0, i: 0, vdc_inv: &[] } }

    #[inline(always)]
    fn new(mut idx: u64, m: u32) -> Self {
        let m2 = m << 1;
        let mut cache = Self {
            d: 0, i: idx << m2,
            vdc_inv: VDC_INV_SOBOL_MATRIX[(m - 1) as usize],
        };
        let mut c = 0;
        while idx != 0 {
            if (idx & 1) == 1 {
                cache.d ^= VDC_SOBOL_MATRIX[(m - 1) as usize][c];
            }
            idx >>= 1; c += 1;
        }
        cache
    }
}
