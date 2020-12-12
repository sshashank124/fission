use super::*;

const SOBOL_NDIM: u32 = 1024;
const SOBOL_SIZE: u32 = 52;
const SOBOL_MATRIX_LEN: usize = (SOBOL_NDIM * SOBOL_SIZE) as usize;
#[allow(clippy::all)]
const SOBOL_MATRIX: [u32; SOBOL_MATRIX_LEN] = include!("sobol.data");

#[allow(clippy::all)]
const VDC_MATRIX: [[u64; 50]; 25] = include!("vdc.data");

#[allow(clippy::all)]
const VDC_INV_MATRIX: [[u64; 52]; 26] = include!("vdc_inv.data");

pub struct Sobol {
    dim:       u32,
    m:         u32,
    cache:     SampleIndexMemo,
    block_pos: I2,
    pixel_pos: I2,
    rng:       Independent,
}

#[derive(Clone)]
struct SampleIndexMemo {
    d:       u64,
    i:       u64,
    vdc_inv: &'static [u64],
}

impl Sobol {
    pub fn new() -> Self {
        Self { dim:       0,
               m:         0,
               cache:     SampleIndexMemo::default(),
               block_pos: I2::ZERO,
               pixel_pos: I2::ZERO,
               rng:       Independent::new(), }
    }

    #[inline(always)]
    fn sample_index(&self) -> u64 {
        if self.m == 0 {
            return 0
        }
        let A2(x, y) = self.pixel_pos - self.block_pos;
        let mut delta = self.cache.d ^ (y as u64) | ((x as u64) << self.m);
        let mut index = self.cache.i;
        let mut c = 0;
        while delta != 0 {
            if (delta & 1) == 1 {
                index ^= self.cache.vdc_inv[c];
            }
            delta >>= 1;
            c += 1;
        }
        index
    }

    #[inline(always)]
    pub fn for_block(&self, i: I, block: &Block) -> Self {
        let Block { pos, dims, .. } = block;
        let res = ceil_pow2_u32(Num::max(dims[X], dims[Y]) as u32);
        let m = log2_ceil_u32(res);
        let cache = SampleIndexMemo::new(i as u64, m);
        Self { dim: 0,
               m,
               cache,
               block_pos: *pos,
               pixel_pos: I2::ZERO,
               rng: self.rng.for_block(i, block) }
    }

    #[inline(always)]
    pub fn for_pixel(&self, pos: I2) -> Self {
        Self { dim:       0,
               m:         self.m,
               cache:     self.cache.clone(),
               block_pos: self.block_pos,
               pixel_pos: pos,
               rng:       self.rng.for_pixel(), }
    }

    #[inline(always)]
    pub fn next_1d(&mut self) -> F {
        if self.dim >= SOBOL_NDIM {
            // eprintln!("Sobol: dim overflow at idx: {}", self.cache.i);
            return self.rng()
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
    pub fn next_2d(&mut self) -> F2 { A2(self.next_1d(), self.next_1d()) }

    #[inline(always)]
    pub fn rng(&mut self) -> F { self.rng.rng() }
}

#[inline(always)]
fn sample_float(idx: u64, dim: u32) -> F {
    sample_sobol(idx, dim) as F * F::FRAC_1_2POW32
}

#[inline(always)]
fn sample_sobol(mut idx: u64, dim: u32) -> u32 {
    let mut res = 0;
    let mut loc = dim * SOBOL_SIZE;
    while idx != 0 {
        if (idx & 1) == 1 {
            res ^= SOBOL_MATRIX[loc as usize];
        }
        idx >>= 1;
        loc += 1;
    }
    res
}

impl SampleIndexMemo {
    fn default() -> Self { Self { d: 0, i: 0, vdc_inv: &[] } }

    #[inline(always)]
    fn new(mut idx: u64, m: u32) -> Self {
        let m2 = m << 1;
        let mut cache = Self { d:       0,
                               i:       idx << m2,
                               vdc_inv: &VDC_INV_MATRIX[(m - 1) as usize], };
        let mut c = 0;
        while idx != 0 {
            if (idx & 1) == 1 {
                cache.d ^= VDC_MATRIX[(m - 1) as usize][c];
            }
            idx >>= 1;
            c += 1;
        }
        cache
    }
}

#[inline(always)]
pub fn ceil_pow2_u32(i: u32) -> u32 {
    1 << (32 - i.saturating_sub(1).leading_zeros())
}

#[inline(always)]
pub fn log2_ceil_u32(i: u32) -> u32 { 31 - i.leading_zeros() }
