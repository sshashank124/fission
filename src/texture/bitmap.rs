use std::convert::TryFrom;
use std::iter::Sum;
use std::ops::Mul;

#[allow(clippy::wildcard_imports)]
use graphite::*;
use image::{GenericImageView, Rgba, io::Reader};
use serde::Deserialize;

use crate::image::bitmap::Bitmap;
use crate::util::config;

impl<A: Copy> Bitmap<A> {
    #[inline] pub fn eval(&self, s: F2) -> A { self[self.rect.dims.map(Conv::<F>::conv) * s] }
}

impl<A> Bitmap<A> where A: Copy + Zero + Mul<F, Output=A> + Sum<A> {
    #[inline] pub fn mean(&self) -> A
    { self.pixels().copied().sum::<A>() * F::of(self.rect.area()).inv() }
}


#[derive(Debug, Deserialize)]
struct BitmapConfig {
    src: String,
}

impl<A> TryFrom<BitmapConfig> for Bitmap<A> where A: ConvFrom<Rgba<u8>> {
    type Error = anyhow::Error;

    fn try_from(bc: BitmapConfig) -> anyhow::Result<Self> {
        let bitmap_path = config::relative_path(bc.src);
        let image = Reader::open(bitmap_path)?.decode()?;
        let dims = conv!(image.dimensions() => U2 => I2);
        let pixels = image.pixels().map(|(_, _, px)| conv!(px => A));
        Ok(Self::from_seq(dims, pixels))
    }
}

pub fn de_from_config<'de, D, A>(de: D) -> Result<Bitmap<A>, D::Error>
where D: serde::Deserializer<'de>,
      A: ConvFrom<Rgba<u8>>
{ BitmapConfig::deserialize(de).and_then(|bc| TryFrom::try_from(bc)
                                                      .map_err(serde::de::Error::custom)) }
