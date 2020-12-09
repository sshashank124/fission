use exr::prelude::rgba_image::*;

use super::*;
use crate::loader::*;

impl Image {
    pub fn save_exr(&self, filename: &str) -> Res<()> {
        let dims: (usize, usize) = self.dims.map(|a| a as usize).into();
        let dims: Vec2<usize> = dims.into();
        ImageInfo::rgb(dims, SampleType::F32)
            .write_pixels_to_file(filename, write_options::high(),
            &|pos: Vec2<usize>| {
                let pixel = self.at(A2(pos.x() as I, pos.y() as I));
                Pixel::rgb(pixel[0], pixel[1], pixel[2])
            }).map_err(|e| e.to_string())
    }
}
