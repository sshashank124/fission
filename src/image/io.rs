use super::*;

impl Image {
    pub fn save_exr(&self, filename: &str) -> Result<(), String> {
        let dims = (self.dims[X] as usize, self.dims[Y] as usize);
        exr::prelude::write_rgb_f32_file(filename, dims,
                                         |x, y| self.at(A2(x, y).into())
                                                    .to_rgb()
                                                    .map(|f: F| f as f32).into()
        ).map_err(|e| e.to_string())
    }
}
