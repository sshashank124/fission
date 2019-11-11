use std::fs::File;
use std::ops::{Deref, DerefMut};

use openexr::{frame_buffer::{FrameBuffer, PixelStruct},
              ScanlineOutputFile,
              header::Header,
              PixelType::{self, FLOAT}};

use crate::types::*;


pub struct Image {
    data: Vec<Color>,
    dims: I2,
}

impl Image {
    pub fn new(dims: I2) -> Self {
        Self {
            data: vec![Color::BLACK; (dims[X] * dims[Y]) as usize],
            dims,
        }
    }

    pub fn as_block(&mut self) -> Block {
        Block {
            pos: I2::ZERO,
            dims: self.dims,
            img: self,
        }
    }

    pub fn save_exr(&self, filename: &str) -> Result<(), String> {
        let mut f = File::create(filename).map_err(|e| e.to_string())?;
        let mut of = self.prepare_file_for_writing(&mut f)?;
        let mut fb = FrameBuffer::new(self.dims[X], self.dims[Y]);

        fb.insert_channels(&["R", "G", "B"], &self.data);
        of.write_pixels(&fb).map_err(|e| e.to_string())
    }

    fn prepare_file_for_writing<'b>(&self, f: &'b mut File)
            -> Result<ScanlineOutputFile<'b>, String> {
        ScanlineOutputFile::new(f, Header::new().set_resolution(self.dims[X],
                                                                self.dims[Y])
                                                .add_channel("R", FLOAT)
                                                .add_channel("G", FLOAT)
                                                .add_channel("B", FLOAT))
                           .map_err(|e| e.to_string())
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
    pub fn blocks<'a>(&'a mut self, dims: I2) -> BlockIter<'a> {
        BlockIter {
            pos: I2::ZERO,
            dims: dims,
            grid: (self.dims + dims - 1) / dims,
            block: self,
        }
    }

    #[inline(always)]
    pub fn pixels<'a>(&'a mut self) -> PixelIter<'a> {
        PixelIter {
            block: self,
            pos: I2::ZERO,
        }
    }

    #[inline(always)]
    pub fn flat_pos(&self) -> I {
        self.pos[Y] * unsafe {&*self.img}.dims[X] + self.pos[X]
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
            let a = self.pos[X];
            self.pos[X] += 1;
            a
        } else {
            self.pos[X] = 1;
            self.pos[Y] += 1;
            0
        };
        if self.pos[Y] < self.grid[Y] {
            let pos = P2(a, self.pos[Y]) * self.dims;
            Some(Block {
                img: self.block.img as *mut Image,
                pos: self.block.pos + pos,
                dims: self.dims.cw_min(self.block.dims - pos),
            })
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

impl<'a> ExactSizeIterator for BlockIter<'a> { }


pub struct Pixel {
    img: *mut Image,
    pub pos: I2,
}

impl Pixel {
    #[inline(always)]
    pub fn flat_pos(&self) -> I {
        self.pos[Y] * unsafe {&*self.img}.dims[X] + self.pos[X]
    }
}

impl Deref for Pixel {
    type Target = Color;
    #[inline(always)]
    fn deref(&self) -> &Color {
        &unsafe {&*self.img}.data[self.flat_pos() as usize]
    }
}

impl DerefMut for Pixel {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Color {
        let pos = self.flat_pos() as usize;
        &mut unsafe {&mut *self.img}.data[pos]
    }
}

pub struct PixelIter<'a> {
    block: &'a mut Block,
    pos: I2,
}

impl<'a> Iterator for PixelIter<'a> {
    type Item = Pixel;
    #[inline(always)]
    fn next(&mut self) -> Option<Pixel> {
        let a = if self.pos[X] < self.block.dims[X] {
            let a = self.pos[X];
            self.pos[X] += 1;
            a
        } else {
            self.pos[X] = 1;
            self.pos[Y] += 1;
            0
        };
        if self.pos[Y] < self.block.dims[Y] {
            let pos = P2(a, self.pos[Y]);
            Some(Pixel {
                img: self.block.img,
                pos: self.block.pos + pos,
            })
        } else {
            None
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.block.dims[Y] - self.pos[Y] - 1) * self.block.dims[X]
                 + (self.block.dims[X] - self.pos[X]);
        (size as usize, Some(size as usize))
    }
}

impl<'a> ExactSizeIterator for PixelIter<'a> { }


unsafe impl PixelStruct for Color {
    #[inline(always)]
    fn channel_count() -> usize {
        3
    }

    #[inline(always)]
    fn channel(i: usize) -> (PixelType, usize) {
        (FLOAT, 4 * i)
    }
 }
