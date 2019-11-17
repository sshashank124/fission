// mod block;
mod io;

use crate::types::*;
use crate::filter::*;


const BLOCK_SIZE: I2 = A2(8, 8);

pub struct Image {
    dims: I2,
    data: Vec<Color>,
    weights: Vec<F>,
    rfilter: ReconstructionFilter,
}

impl Image {
    #[inline(always)]
    pub fn new(dims: I2, rfilter: ReconstructionFilter) -> Self {
        let len = (dims[X] * dims[Y]) as usize;
        Self {
            dims,
            data: vec![Color::BLACK; len],
            weights: vec![0.; len],
            rfilter,
        }
    }

    #[inline(always)]
    pub fn as_block(&mut self) -> Block {
        Block { pos: I2::ZERO, dims: self.dims, img: self }
    }

    #[inline(always)]
    pub fn flat_pos(&self, pos: I2) -> usize {
        (pos[Y] * self.dims[X] + pos[X]) as usize
    }
}

pub struct Block {
    img: *mut Image,
    pub pos: I2,
    pub dims: I2,
}

unsafe impl Send for Block { }

impl Block {
    #[inline(always)]
    pub fn put(&mut self, offset: F2, color: Color) {
        let img = unsafe { &mut *self.img };

        let offset = offset - A2(0.5, 0.5) - F2::from(self.pos);
        let r = img.rfilter.radius();
        let lo = offset - r; let hi = offset + r;
        let (lx, ly) = (Num::max(lo[X].ceili(), 0),
                        Num::max(lo[Y].ceili(), 0));
        let (hx, hy) = (Num::min(hi[X].floori(), self.dims[X] - 1),
                        Num::min(hi[Y].floori(), self.dims[Y] - 1));

        for y in ly..=hy { for x in lx..=hx {
            let w = img.rfilter.eval(Num::abs(x as F - offset[X]))
                  * img.rfilter.eval(Num::abs(y as F - offset[Y]));
            let loc = img.flat_pos(self.pos + A2(x, y));
            img.data[loc] += color * w;
            img.weights[loc] += w;
        } }
    }

    #[inline(always)]
    pub fn blocks(&mut self) -> BlockIter<'_> {
        let dims = BLOCK_SIZE;
        BlockIter {
            pos: I2::ZERO,
            dims,
            grid: (self.dims + dims - 1) / dims,
            block: self,
        }
    }

    #[inline(always)]
    pub fn pixels(&self) -> PixelIter {
        PixelIter { block_pos: self.pos, block_dims: self.dims, pos: I2::ZERO }
    }
}

pub struct BlockIter<'a> {
    block: &'a mut Block,
    pos: I2,
    dims: I2,
    grid: I2,
}

impl<'a> Iterator for BlockIter<'a> {
    type Item = Block;
    #[inline(always)]
    fn next(&mut self) -> Option<Block> {
        let a = if self.pos[X] < self.grid[X] {
            let a = self.pos[X]; self.pos[X] += 1; a
        } else { self.pos[X] = 1; self.pos[Y] += 1; 0 };
        if self.pos[Y] < self.grid[Y] {
            let pos = A2(a, self.pos[Y]) * self.dims;
            Some(Block {
                img: self.block.img as *mut Image,
                pos: self.block.pos + pos,
                dims: self.dims.zip(self.block.dims - pos, Num::min),
            })
        } else { None }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.grid[Y] - self.pos[Y] - 1) * self.grid[X]
                 + (self.grid[X] - self.pos[X]);
        (size as usize, Some(size as usize))
    }
}

impl<'a> ExactSizeIterator for BlockIter<'a> { }


pub struct PixelIter {
    block_pos: I2,
    block_dims: I2,
    pos: I2,
}

impl Iterator for PixelIter {
    type Item = I2;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let a = if self.pos[X] < self.block_dims[X] {
            let a = self.pos[X]; self.pos[X] += 1; a
        } else { self.pos[X] = 1; self.pos[Y] += 1; 0 };
        if self.pos[Y] < self.block_dims[Y] {
            let pos = A2(a, self.pos[Y]);
            Some(self.block_pos + pos)
        } else { None }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.block_dims[Y] - self.pos[Y] - 1) * self.block_dims[X]
                 + (self.block_dims[X] - self.pos[X]);
        (size as usize, Some(size as usize))
    }
}

impl ExactSizeIterator for PixelIter { }
