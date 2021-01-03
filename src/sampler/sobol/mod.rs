#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

use crate::image::rect::Rect;
use crate::sampler::independent::Independent;

const SOBOL_NDIM: u32 = 1024;
const SOBOL_SIZE: u32 = 52;
const SOBOL_MATRIX_LEN: usize = (SOBOL_NDIM * SOBOL_SIZE) as usize;
#[allow(clippy::all)]
const SOBOL_MATRIX: [u32; SOBOL_MATRIX_LEN] = include!("sobol.data");

#[allow(clippy::all)]
const VDC_MATRIX: [[u64; 50]; 25] = include!("vdc.data");

#[allow(clippy::all)]
const VDC_INV_MATRIX: [[u64; 52]; 26] = include!("vdc_inv.data");

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Sobol {
    #[serde(skip)] dim:       u32,
    #[serde(skip)] m:         u32,
    #[serde(skip)] cache:     SampleIndexMemo,
    #[serde(skip)] block_pos: I2,
    #[serde(skip)] pixel_pos: I2,
    #[serde(skip)] rng:       Independent,
}

#[derive(Clone, Debug, Default)]
struct SampleIndexMemo {
    d:       u64,
    i:       u64,
    vdc_inv: &'static [u64],
}

impl Sobol {
    #[inline] fn sample_index(&self) -> u64 {
        if self.m == 0 {
            return 0
        }
        let pos = self.pixel_pos - self.block_pos;
        let mut delta = self.cache.d ^ (pos[Y] as u64)
                      | ((pos[X] as u64) << self.m);
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

    #[inline] pub fn for_rect(pass: I, rect: &Rect) -> Self {
        let res = ceil_pow2_u32(rect.length() as u32);
        let m = log2_ceil_u32(res);
        let cache = SampleIndexMemo::new(pass as u64, m);
        Self { dim: 0,
               m,
               cache,
               block_pos: rect.pos,
               pixel_pos: I2::ZERO,
               rng: Independent::for_rect(pass, rect) }
    }

    #[inline] pub fn prepare_for_pixel(&mut self, pos: I2) {
        self.dim = 0;
        self.pixel_pos = pos;
    }

    #[inline] fn next_1d(&mut self) -> F {
        if self.dim >= SOBOL_NDIM {
            eprintln!("Sobol: dim overflow at idx: {}", self.cache.i);
            return self.rng()
        }

        let mut f = sample_float(self.sample_index(), self.dim);

        if self.dim < 2 {
            f = f.mul_add((1 << self.m) as F,
                          self.block_pos[self.dim as I] as F);
            f = Num::clamp_unit(f - self.pixel_pos[self.dim as I] as F);
        }

        self.dim += 1;
        f
    }

    #[inline] pub fn next_2d(&mut self) -> F2
    { A2(self.next_1d(), self.next_1d()) }

    #[inline] pub fn rng(&mut self) -> F { self.rng.rng() }
}

#[inline] fn sample_float(idx: u64, dim: u32) -> F {
    sample_sobol(idx, dim) as F * F::FRAC_1_2POW32
}

#[inline] const fn sample_sobol(mut idx: u64, dim: u32) -> u32 {
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
    #[inline] const fn new(mut idx: u64, m: u32) -> Self {
        let m2 = m << 1;
        let mut cache = Self { d:       0,
                               i:       idx << m2,
                               vdc_inv: &VDC_INV_MATRIX[(m - 1) as usize] };
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

#[inline] pub const fn ceil_pow2_u32(i: u32) -> u32
{ 1 << (32 - i.saturating_sub(1).leading_zeros()) }

#[inline] pub const fn log2_ceil_u32(i: u32) -> u32 { 31 - i.leading_zeros() }
