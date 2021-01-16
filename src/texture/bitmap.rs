#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

use crate::color::Color;
use crate::image::bitmap::Bitmap;

impl<A> Bitmap<A> where A: Zero {
    #[inline] pub fn eval(&self, _s: F2) -> A { A::ZERO }
}

impl<A> Bitmap<A> where A: Zero {
    #[inline] pub fn mean(&self) -> A { A::ZERO }
}


#[derive(Debug, Deserialize)]
struct BitmapConfig {
    src: String,
}

impl From<BitmapConfig> for Bitmap<Color> {
    fn from(bc: BitmapConfig) -> Self {
        Bitmap::new(A2(0, 0))
    }
}

pub fn de_from_config<'de, D, A>(de: D) -> Result<Bitmap<A>, D::Error>
where D: serde::Deserializer<'de>
{ BitmapConfig::deserialize(de).map(From::from) }
