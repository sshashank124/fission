mod diffuse;
mod mirror;

use crate::geometry::*;
use crate::texture::*;

pub use diffuse::Diffuse;
pub use mirror::Mirror;


pub trait Bxdf {
    fn eval(&self, wi: V, wo: V, uv: F2) -> Color;
    fn sample(&self, wo: V, uv: F2, s: F2) -> (Color, V);
}

pub enum Bsdf {
    Diffuse(Diffuse),
    Mirror(Mirror),
}

impl Bxdf for Bsdf {
    #[inline(always)] fn eval(&self, wi: V, wo: V, uv: F2) -> Color {
        match self {
            Self::Diffuse(f) => f.eval(wi, wo, uv),
            Self::Mirror(f) => f.eval(wi, wo, uv),
        }
    }

    #[inline(always)] fn sample(&self, wo: V, uv: F2, s: F2) -> (Color, V) {
        match self {
            Self::Diffuse(f) => f.sample(wo, uv, s),
            Self::Mirror(f) => f.sample(wo, uv, s),
        }
    }
}

impl From<Diffuse> for Bsdf
{ #[inline(always)] fn from(f: Diffuse) -> Self { Self::Diffuse(f) } }

impl From<Mirror> for Bsdf
{ #[inline(always)] fn from(f: Mirror) -> Self { Self::Mirror(f) } }

impl Zero for Bsdf { const ZERO: Self = Self::Diffuse(Diffuse::ZERO); }
