mod diffuse;

use crate::geometry::*;
use crate::texture::*;

pub use diffuse::Diffuse;


pub type BsdfQuery = (V, V, F2);  // (wi, wo, uv)

pub trait Bxdf {
    fn eval(&self, bq: BsdfQuery) -> Color;
}

pub enum Bsdf {
    Diffuse(Diffuse),
}

impl Bxdf for Bsdf {
    #[inline(always)] fn eval(&self, bq: BsdfQuery) -> Color {
        match self {
            Self::Diffuse(f) => f.eval(bq),
        }
    }
}

impl From<Diffuse> for Bsdf
{ #[inline(always)] fn from(f: Diffuse) -> Self { Self::Diffuse(f) } }

impl Zero for Bsdf { const ZERO: Self = Self::Diffuse(Diffuse::ZERO); }
