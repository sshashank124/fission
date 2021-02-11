pub mod bitmap;
pub mod pixel;
pub mod rect;

use exr::prelude::write_rgb_f32_file;
#[allow(clippy::wildcard_imports)]
use graphite::*;

use bitmap::Bitmap;
use pixel::Pixel;

use crate::color::{Color, Rgb};

const BLOCK_SIZE: I2 = A2(32, 32);

pub type Image = Bitmap<Pixel>;

macro_rules! conv {
    ($expr:expr) => { $expr };
    ($expr:expr => $t:ty $(=> $tt:ty)*) => { conv!(<$t>::of($expr) $(=> $tt)*) };
}

impl Image {
    pub fn save_exr(&self, filename: &str) -> anyhow::Result<()> {
        let dims = conv!(self.rect.dims => A2<usize> => (usize, usize));
        write_rgb_f32_file(filename, dims, |x, y| {
            let idx = conv!(A2(x, y) => I2);
            conv!(self[idx] => Color => Rgb => F3 => A3<f32> => (f32, f32, f32))
        })?;
        Ok(())
    }
}
