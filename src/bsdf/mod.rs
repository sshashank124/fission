mod dielectric;
mod diffuse;
mod fresnel;
mod microfacet;
mod mirror;

use crate::geometry::*;
use crate::texture::*;
use fresnel::*;

pub use dielectric::Dielectric;
pub use diffuse::Diffuse;
pub use microfacet::Microfacet;
pub use mirror::Mirror;


pub trait Bxdf {
    // BSDF * cos(theta)
    fn eval(&self, wi: V, wo: V, uv: F2) -> Color;

    // (color, wo, pdf, specular)
    fn sample(&self, wi: V, uv: F2, s: F2) -> (Color, V, F, bool);

    fn pdf(&self, wi: V, wo: V) -> F;

    fn is_delta(&self) -> bool;
}

pub enum Bsdf {
    Dielectric(Dielectric),
    Diffuse(Diffuse),
    Microfacet(Microfacet),
    Mirror(Mirror),
}

impl Bxdf for Bsdf {
    #[inline(always)] fn eval(&self, wi: V, wo: V, uv: F2) -> Color {
        match self {
            Self::Dielectric(f) => f.eval(wi, wo, uv),
            Self::Diffuse(f) => f.eval(wi, wo, uv),
            Self::Microfacet(f) => f.eval(wi, wo, uv),
            Self::Mirror(f) => f.eval(wi, wo, uv),
        }
    }

    #[inline(always)]
    fn sample(&self, wi: V, uv: F2, s: F2) -> (Color, V, F, bool) {
        match self {
            Self::Dielectric(f) => f.sample(wi, uv, s),
            Self::Diffuse(f) => f.sample(wi, uv, s),
            Self::Microfacet(f) => f.sample(wi, uv, s),
            Self::Mirror(f) => f.sample(wi, uv, s),
        }
    }

    #[inline(always)] fn pdf(&self, wi: V, wo: V) -> F {
        match self {
            Self::Dielectric(f) => f.pdf(wi, wo),
            Self::Diffuse(f) => f.pdf(wi, wo),
            Self::Microfacet(f) => f.pdf(wi, wo),
            Self::Mirror(f) => f.pdf(wi, wo),
        }
    }

    #[inline(always)] fn is_delta(&self) -> bool {
        match self {
            Self::Dielectric(f) => f.is_delta(),
            Self::Diffuse(f) => f.is_delta(),
            Self::Microfacet(f) => f.is_delta(),
            Self::Mirror(f) => f.is_delta(),
        }
    }
}

impl From<Dielectric> for Bsdf
{ fn from(f: Dielectric) -> Self { Self::Dielectric(f) } }

impl From<Diffuse> for Bsdf
{ fn from(f: Diffuse) -> Self { Self::Diffuse(f) } }

impl From<Microfacet> for Bsdf
{ fn from(f: Microfacet) -> Self { Self::Microfacet(f) } }

impl From<Mirror> for Bsdf
{ fn from(f: Mirror) -> Self { Self::Mirror(f) } }

impl Zero for Bsdf { const ZERO: Self = Self::Diffuse(Diffuse::ZERO); }
