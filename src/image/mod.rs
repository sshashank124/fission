mod io;

use crate::core::*;

const BLOCK_SIZE: I2 = A2(8, 8);

pub struct Image {
    dims:    I2,
    data:    Vec<Color>,
    weights: Vec<F>,
}

impl Image {
    pub fn new(dims: I2) -> Self {
        let len = (dims[X] * dims[Y]) as usize;
        let data = vec![Color::BLACK; len];
        let weights = vec![0.; len];
        Self { dims, data, weights }
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
    img:      *mut Image,
    pub pos:  I2,
    pub dims: I2,
}

unsafe impl Send for Block {}

impl Block {
    #[inline(always)]
    pub fn put(&mut self, pos: F2, color: Color) {
        let img = unsafe { &mut *self.img };
        let pos = pos.map(F::floori).zip(self.pos + self.dims - 1, Num::min);
        let loc = img.flat_pos(pos);
        img.data[loc] += color;
        img.weights[loc] += 1.;
    }

    #[inline(always)]
    pub fn blocks(&mut self) -> impl Iterator<Item = Block> + '_ {
        let dims = BLOCK_SIZE;
        BlockIter { pos: I2::ZERO,
                    dims,
                    grid: (self.dims + dims - 1) / dims,
                    block: self }
    }

    #[inline(always)]
    pub fn pixels(&self) -> impl Iterator<Item = I2> {
        PixelIter { block_pos:  self.pos,
                    block_dims: self.dims,
                    pos:        I2::ZERO, }
    }
}

pub struct BlockIter<'a> {
    block: &'a mut Block,
    pos:   I2,
    dims:  I2,
    grid:  I2,
}

impl<'a> Iterator for BlockIter<'a> {
    type Item = Block;
    #[inline(always)]
    fn next(&mut self) -> Option<Block> {
        let a = if self.pos[X] < self.grid[X] {
            let a = self.pos[X];
            self.pos[X] += 1;
            a
        } else {
            self.pos[X] = 1;
            self.pos[Y] += 1;
            0
        };
        if self.pos[Y] < self.grid[Y] {
            let pos = A2(a, self.pos[Y]) * self.dims;
            let dims = self.dims.zip(self.block.dims - pos, Num::min);
            Some(Block { img: self.block.img as *mut Image,
                         pos: self.block.pos + pos,
                         dims })
        } else {
            None
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.grid[Y] - self.pos[Y] - 1) * self.grid[X]
                   + (self.grid[X] - self.pos[X]);
        (size as usize, Some(size as usize))
    }
}

impl<'a> ExactSizeIterator for BlockIter<'a> {}

pub struct PixelIter {
    block_pos:  I2,
    block_dims: I2,
    pos:        I2,
}

impl Iterator for PixelIter {
    type Item = I2;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let a = if self.pos[X] < self.block_dims[X] {
            let a = self.pos[X];
            self.pos[X] += 1;
            a
        } else {
            self.pos[X] = 1;
            self.pos[Y] += 1;
            0
        };
        if self.pos[Y] < self.block_dims[Y] {
            let pos = A2(a, self.pos[Y]);
            Some(self.block_pos + pos)
        } else {
            None
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.block_dims[Y] - self.pos[Y] - 1) * self.block_dims[X]
                   + (self.block_dims[X] - self.pos[X]);
        (size as usize, Some(size as usize))
    }
}

impl ExactSizeIterator for PixelIter {}
