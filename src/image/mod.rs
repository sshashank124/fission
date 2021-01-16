pub mod bitmap;
pub mod pixel;
pub mod rect;

use exr::prelude::write_rgb_f32_file;
#[allow(clippy::wildcard_imports)]
use graphite::*;

use bitmap::Bitmap;
use pixel::Pixel;

const BLOCK_SIZE: I2 = A2(32, 32);

pub type Image = Bitmap<Pixel>;

impl Image {
    pub fn save_exr(&self, filename: &str) -> anyhow::Result<()> {
        let dims: (usize, usize) = A2::of(self.rect.dims).into();
        write_rgb_f32_file(filename, dims, |x, y| {
            let color = self[I2::of(A2(x, y))].to_color().to_rgb().0;
            color.into()
        })?;
        Ok(())
    }
}
