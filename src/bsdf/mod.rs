mod diffuse;
mod microfacet;
mod mirror;

use crate::geometry::*;
use crate::texture::*;

pub use diffuse::Diffuse;
pub use microfacet::Microfacet;
pub use mirror::Mirror;


pub trait Bxdf {
    fn eval(&self, wi: V, wo: V, uv: F2) -> Color;
    fn sample_dir(&self, wi: V, s: F2) -> V;
    fn pdf(&self, wi: V, wo: V) -> F;

    #[inline(always)] fn sample(&self, wi: V, uv: F2, s: F2) -> (Color, V) {
        let wo = self.sample_dir(wi, s);
        let p = self.pdf(wi, wo);
        (if p <= 0. { Color::BLACK } else {
            self.eval(wi, wo, uv) / p * Frame::ct(*wo)
        }, wo)
    }
}

pub enum Bsdf {
    Diffuse(Diffuse),
    Microfacet(Microfacet),
    Mirror(Mirror),
}

impl Bxdf for Bsdf {
    #[inline(always)] fn eval(&self, wi: V, wo: V, uv: F2) -> Color {
        match self {
            Self::Diffuse(f) => f.eval(wi, wo, uv),
            Self::Microfacet(f) => f.eval(wi, wo, uv),
            Self::Mirror(f) => f.eval(wi, wo, uv),
        }
    }

    #[inline(always)] fn sample(&self, wi: V, uv: F2, s: F2) -> (Color, V) {
        match self {
            Self::Diffuse(f) => f.sample(wi, uv, s),
            Self::Microfacet(f) => f.sample(wi, uv, s),
            Self::Mirror(f) => f.sample(wi, uv, s),
        }
    }

    #[inline(always)] fn sample_dir(&self, wi: V, s: F2) -> V {
        match self {
            Self::Diffuse(f) => f.sample_dir(wi, s),
            Self::Microfacet(f) => f.sample_dir(wi, s),
            Self::Mirror(f) => f.sample_dir(wi, s),
        }
    }

    #[inline(always)] fn pdf(&self, wi: V, wo: V) -> F {
        match self {
            Self::Diffuse(f) => f.pdf(wi, wo),
            Self::Microfacet(f) => f.pdf(wi, wo),
            Self::Mirror(f) => f.pdf(wi, wo),
        }
    }
}

impl From<Diffuse> for Bsdf
{ #[inline(always)] fn from(f: Diffuse) -> Self { Self::Diffuse(f) } }

impl From<Microfacet> for Bsdf
{ #[inline(always)] fn from(f: Microfacet) -> Self { Self::Microfacet(f) } }

impl From<Mirror> for Bsdf
{ #[inline(always)] fn from(f: Mirror) -> Self { Self::Mirror(f) } }

impl Zero for Bsdf { const ZERO: Self = Self::Diffuse(Diffuse::ZERO); }
