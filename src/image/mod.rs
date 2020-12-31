mod pixel;
mod rect;

use std::ops::{AddAssign, Index, IndexMut};

use exr::prelude::write_rgb_f32_file;

use crate::prelude::*;
use pixel::Pixel;
pub use rect::Rect;


pub type Image = Block;

const BLOCK_SIZE: I2 = A2(16, 16);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Block {
        data: Vec<Pixel>,
    pub rect: Rect,
}

impl Block {
    #[inline(always)] pub fn new(dims: I2) -> Self
    { Self::from_iter(Rect::new(I2::ZERO, dims), std::iter::empty()) }

    #[inline(always)] pub fn from_iter<It>(rect: Rect, it: It) -> Self
        where It: IntoIterator<Item=(F2, Color)>
    {
        let data = vec![Pixel::ZERO; rect.area() as usize];
        let mut block = Self { data, rect };
        it.into_iter().for_each(|(pos, color)| block[pos] += color);
        block
    }

    pub fn save_exr(&self, filename: &str) -> Result<()> {
        let dims: (usize, usize) = A2::from(self.rect.dims).into();
        write_rgb_f32_file(filename, dims, |x, y| self[I2::from(A2(x, y))]
                                                      .eval().to_rgb()
                                                      .map(|f| f as f32)
                                                      .into())?;
        Ok(())
    }
}

impl AddAssign<Block> for Block {
    #[inline(always)] fn add_assign(&mut self, block: Block)
    { block.rect.positions().for_each(|pos| self[pos] += block[pos]); }
}

impl Index<I2> for Block {
    type Output = Pixel;
    #[inline(always)] fn index(&self, pos: I2) -> &Pixel
    { &self.data[self.rect.flatten_abs_pos(pos) as usize] }
}

impl IndexMut<I2> for Block {
    #[inline(always)] fn index_mut(&mut self, pos: I2) -> &mut Pixel {
        let idx = self.rect.flatten_abs_pos(pos) as usize;
        &mut self.data[idx]
    }
}

impl Index<F2> for Block {
    type Output = Pixel;
    #[inline(always)] fn index(&self, pos: F2) -> &Pixel
    { &self[pos.map(F::floori)] }
}

impl IndexMut<F2> for Block {
    #[inline(always)] fn index_mut(&mut self, pos: F2) -> &mut Pixel
    { &mut self[pos.map(F::floori)] }
}
