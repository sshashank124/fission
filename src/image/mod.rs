pub mod pixel;
pub mod rect;

use std::iter::Sum;
use std::ops::{AddAssign, Index, IndexMut};

use exr::prelude::write_rgb_f32_file;
#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::{Deserialize, Serialize};

use crate::color::Color;

use pixel::Pixel;
use rect::Rect;

pub type Image = Block;

const BLOCK_SIZE: I2 = A2(64, 64);

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Block {
        data: Box<[Pixel]>,
    pub rect: Rect,
}

impl Block {
    #[inline] pub fn new(dims: I2) -> Self
    { Self::from_iter(Rect::new(I2::ZERO, dims), std::iter::empty()) }

    #[inline] pub fn from_iter<It>(rect: Rect, it: It) -> Self
        where It: IntoIterator<Item=(F2, Color)>
    {
        let data = vec![Pixel::ZERO; rect.area().conv()].into_boxed_slice();
        let mut block = Self { data, rect };
        it.into_iter().for_each(|(pos, color)| block[pos] += color);
        block
    }

    #[inline] pub fn pixels(&self) -> impl Iterator<Item=&Pixel>
    { self.data.iter() }

    pub fn save_exr(&self, filename: &str) -> anyhow::Result<()> {
        let dims: (usize, usize) = A2::of(self.rect.dims).into();
        write_rgb_f32_file(filename, dims, |x, y| {
            let color = self[I2::of(A2(x, y))].to_color().to_rgb().0;
            color.into()
        })?;
        Ok(())
    }
}

impl AddAssign for Block {
    #[inline] fn add_assign(&mut self, block: Self)
    { block.rect.positions().for_each(|pos| self[pos] += block[pos]); }
}

impl Index<I2> for Block {
    type Output = Pixel;
    #[inline] fn index(&self, pos: I2) -> &Pixel
    { &self.data[usize::of(self.rect.flatten_abs_pos(pos))] }
}

impl IndexMut<I2> for Block {
    #[inline] fn index_mut(&mut self, pos: I2) -> &mut Pixel {
        let idx = usize::of(self.rect.flatten_abs_pos(pos));
        &mut self.data[idx]
    }
}

impl Index<F2> for Block {
    type Output = Pixel;
    #[inline] fn index(&self, pos: F2) -> &Pixel
    { &self[pos.map(F::floori)] }
}

impl IndexMut<F2> for Block {
    #[inline] fn index_mut(&mut self, pos: F2) -> &mut Pixel
    { &mut self[pos.map(F::floori)] }
}

impl Sum for Block {
    fn sum<It>(it: It) -> Self where It: Iterator<Item=Block> {
        let mut it = it.filter(|b| b.rect.dims != I2::ZERO);
        match it.next() {
            None => Self::default(),
            Some(mut acc) => { it.for_each(|b| acc += b); acc }
        }
    }
}
