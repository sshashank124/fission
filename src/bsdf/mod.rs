mod diffuse;

use crate::geometry::*;

pub use diffuse::Diffuse;


pub trait BXDF {
    fn eval(&self, wi: V, wo: V) -> Color;
}

pub enum BSDF {
    Diffuse(Diffuse),
}

impl BXDF for BSDF {
    #[inline(always)] fn eval(&self, wi: V, wo: V) -> Color {
        match self {
            Self::Diffuse(f) => f.eval(wi, wo),
        }
    }
}

impl From<Diffuse> for BSDF
{ #[inline(always)] fn from(f: Diffuse) -> Self { Self::Diffuse(f) } }

impl Zero for BSDF { const ZERO: Self = Self::Diffuse(Diffuse::ONE); }
