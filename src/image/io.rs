use std::fs::File;

use openexr::{frame_buffer::{FrameBuffer, PixelStruct},
              ScanlineOutputFile,
              header::Header,
              PixelType::{self, FLOAT}};

use super::*;
use crate::loader::*;


impl Image {
    pub fn save_exr(&self, filename: &str) -> Res<()> {
        let data = self.data.iter().zip(self.weights.iter())
            .map(|(&c, &w)| (c / if w > 0. { w } else { 1. })
                                .map(|f| f as f32))
            .collect::<Vec<_>>();

        let mut f = File::create(filename).map_err(|e| e.to_string())?;
        let mut of = self.prepare_file_for_writing(&mut f)?;
        let mut fb = FrameBuffer::new(self.dims[X] as u32,
                                      self.dims[Y] as u32);

        fb.insert_channels(&["R", "G", "B"], &data);
        of.write_pixels(&fb).map_err(|e| e.to_string())
    }

    fn prepare_file_for_writing<'a>(&self, f: &'a mut File)
            -> Result<ScanlineOutputFile<'a>, String> {
        ScanlineOutputFile::new(f, Header::new()
                                          .set_resolution(self.dims[X] as u32,
                                                          self.dims[Y] as u32)
                                          .add_channel("R", FLOAT)
                                          .add_channel("G", FLOAT)
                                          .add_channel("B", FLOAT))
                           .map_err(|e| e.to_string())
    }
}

unsafe impl PixelStruct for A3<f32> {
    fn channel_count() -> usize { 3 }
    fn channel(i: usize) -> (PixelType, usize) { (FLOAT, 4 * i) }
}
