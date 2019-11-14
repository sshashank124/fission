mod sobol;
mod vdc;
mod vdc_inv;

use super::*;
use sobol::*;
use vdc::*;
use vdc_inv::*;


pub struct Sobol {
    idx: u64,
    dim: u32,
    res: u32,
    log_res: u32,
    block_pos: I2,
    pixel_pos: I2,
    rng: Independent,
}

impl Sobol {
    #[inline(always)] pub fn new() -> Self {
        Self {
            idx: 0,
            dim: 0,
            res: 0,
            log_res: 0,
            block_pos: I2::ZERO,
            pixel_pos: I2::ZERO,
            rng: Independent::new(),
        }
    }

    #[inline(always)]
    fn sample_index(&self) -> u64 {
        let m = self.log_res;
        if m == 0 { return 0; }

        let m2 = m << 1;
        let mut f = self.idx;
        let mut i = f << m2; let mut c = 0; let mut d = 0;
        while f != 0 {
            if (f & 1) == 1 { d ^= VDC_SOBOL_MATRIX[(m - 1) as usize][c]; }
            f >>= 1; c += 1;
        }
        let P2(x, y) = self.pixel_pos - self.block_pos;
        c = 0; d ^= (y as u64) | ((x as u64) << m);
        while d != 0 {
            if (d & 1) == 1 { i ^= VDC_INV_SOBOL_MATRIX[(m - 1) as usize][c]; }
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
        let log_res = log2_ceil_u32(res);
        Self {
            idx: i as u64,
            dim: 0,
            res,
            log_res,
            block_pos: *pos,
            pixel_pos: I2::ZERO,
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
            f = (f * self.res as F) + self.block_pos[self.dim as I] as F;
            f = Num::clamp_unit(f - self.pixel_pos[self.dim as I] as F);
        }

        self.dim += 1;
        f
    }

    #[inline(always)]
    fn next_2d(&mut self) -> F2 {
        P2(self.next_1d(), self.next_1d())
    }

    #[inline(always)] fn rng(&mut self) -> F { self.rng.rng() }
}

#[inline(always)]
fn sample_float(idx: u64, dim: u32) -> F {
    sample_sobol(idx, dim) as F * F::FRAC_1_2POW32
}

#[inline(always)]
fn sample_sobol(mut idx: u64, dim: u32) -> u32 {
    let mut loc = dim * SOBOL_SIZE;
    let mut res = 0;
    while idx != 0 {
        if (idx & 1) == 1 { res ^= SOBOL_MATRIX[loc as usize]; }
        idx >>= 1;
        loc += 1;
    }
    res
}
