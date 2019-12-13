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


pub trait BXDF {
    // BSDF * cos(theta)
    fn eval(&self, wi: V, wo: V, uv: F2) -> Color;

    // (color, wo, pdf, specular)
    fn sample(&self, wi: V, uv: F2, s: F2) -> (Color, V, F, bool);

    fn pdf(&self, wi: V, wo: V) -> F;

    fn is_delta(&self) -> bool;
}

pub enum BSDF {
    None,
    Dielectric(Dielectric),
    Diffuse(Diffuse),
    Microfacet(Microfacet),
    Mirror(Mirror),
}

impl BSDF {
    #[inline(always)] pub fn exists(&self) -> bool
    { match self { Self::None => false, _ => true } }
}

impl BXDF for BSDF {
    #[inline(always)] fn eval(&self, wi: V, wo: V, uv: F2) -> Color {
        match self {
            Self::None => Color::BLACK,
            Self::Dielectric(f) => f.eval(wi, wo, uv),
            Self::Diffuse(f) => f.eval(wi, wo, uv),
            Self::Microfacet(f) => f.eval(wi, wo, uv),
            Self::Mirror(f) => f.eval(wi, wo, uv),
        }
    }

    #[inline(always)]
    fn sample(&self, wi: V, uv: F2, s: F2) -> (Color, V, F, bool) {
        match self {
            Self::None => (Color::BLACK, V::ZERO, 0., false),
            Self::Dielectric(f) => f.sample(wi, uv, s),
            Self::Diffuse(f) => f.sample(wi, uv, s),
            Self::Microfacet(f) => f.sample(wi, uv, s),
            Self::Mirror(f) => f.sample(wi, uv, s),
        }
    }

    #[inline(always)] fn pdf(&self, wi: V, wo: V) -> F {
        match self {
            Self::None => 0.,
            Self::Dielectric(f) => f.pdf(wi, wo),
            Self::Diffuse(f) => f.pdf(wi, wo),
            Self::Microfacet(f) => f.pdf(wi, wo),
            Self::Mirror(f) => f.pdf(wi, wo),
        }
    }

    #[inline(always)] fn is_delta(&self) -> bool {
        match self {
            Self::None => false,
            Self::Dielectric(f) => f.is_delta(),
            Self::Diffuse(f) => f.is_delta(),
            Self::Microfacet(f) => f.is_delta(),
            Self::Mirror(f) => f.is_delta(),
        }
    }
}

impl From<Dielectric> for BSDF
{ fn from(f: Dielectric) -> Self { Self::Dielectric(f) } }

impl From<Diffuse> for BSDF
{ fn from(f: Diffuse) -> Self { Self::Diffuse(f) } }

impl From<Microfacet> for BSDF
{ fn from(f: Microfacet) -> Self { Self::Microfacet(f) } }

impl From<Mirror> for BSDF
{ fn from(f: Mirror) -> Self { Self::Mirror(f) } }

impl Zero for BSDF { const ZERO: Self = Self::None; }
