use std::fs::File;
use std::ops::{Index, IndexMut};

use openexr::{frame_buffer::{FrameBuffer, PixelStruct},
              ScanlineOutputFile,
              header::Header,
              PixelType::{self, FLOAT}};

use crate::types::*;


pub struct Image {
    w: I,
    h: I,
    data: Vec<Color>,
}

impl Image {
    pub fn new(P2(w, h): I2) -> Image {
        Image {
            w,
            h,
            data: vec![Color::BLACK; (w * h) as usize],
        }
    }

    #[inline]
    pub fn flatten(&self, P2(x, y): I2) -> I {
        y * self.w + x
    }

    #[inline]
    pub fn pixels(&self) -> impl Iterator<Item=I2> {
        CartesianProduct {
            a_max: self.w,
            b_max: self.h,
            a: 0,
            b: 0,
        }
    }

    pub fn save_exr(&self, filename: &str) -> Result<(), String> {
        let mut f = File::create(filename).map_err(|e| e.to_string())?;
        let mut of = self.prepare_file_for_writing(&mut f)?;
        let mut fb = FrameBuffer::new(self.w, self.h);

        fb.insert_channels(&["R", "G", "B"], &self.data);
        of.write_pixels(&fb).map_err(|e| e.to_string())
    }

    fn prepare_file_for_writing<'a>(&self, f: &'a mut File)
            -> Result<ScanlineOutputFile<'a>, String> {
        ScanlineOutputFile::new(f, Header::new().set_resolution(self.w, self.h)
                                                .add_channel("R", FLOAT)
                                                .add_channel("G", FLOAT)
                                                .add_channel("B", FLOAT))
                           .map_err(|e| e.to_string())
    }
}

impl Index<I2> for Image {
    type Output = Color;
    #[inline]
    fn index(&self, p: I2) -> &Color {
        &self.data[self.flatten(p) as usize]
    }
}

impl IndexMut<I2> for Image {
    #[inline]
    fn index_mut(&mut self, p: I2) -> &mut Color {
        let i = self.flatten(p) as usize;
        &mut self.data[i]
    }
}

pub struct CartesianProduct {
    a_max: I,
    b_max: I,
    a: I,
    b: I,
}

impl Iterator for CartesianProduct {
    type Item = I2;
    #[inline]
    fn next(&mut self) -> Option<I2> {
        let an = if self.a < self.a_max {
            self.a += 1;
            self.a - 1
        } else {
            self.a = 0;
            self.b += 1;
            self.a
        };
        if self.b < self.b_max {
            Some(P2(an, self.b))
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.b_max - self.b - 1) * self.a_max
                 + (self.a_max - self.a);
        (size as usize, Some(size as usize))
    }
}

impl ExactSizeIterator for CartesianProduct { }

unsafe impl PixelStruct for Color {
    #[inline]
    fn channel_count() -> usize {
        3
    }

    #[inline]
    fn channel(i: usize) -> (PixelType, usize) {
        (FLOAT, 4 * i)
    }
 }
