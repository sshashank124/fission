mod dielectric;
mod diffuse;
mod fresnel;
mod microfacet;
mod mirror;

use crate::prelude::*;
use crate::texture::*;
use fresnel::*;

pub use dielectric::Dielectric;
pub use diffuse::Diffuse;
pub use microfacet::Microfacet;
pub use mirror::Mirror;

#[derive(Debug, Deserialize)]
#[serde(tag="type", rename_all="snake_case")]
pub enum BSDF {
    Dielectric(Dielectric),
    Diffuse(Diffuse),
    Microfacet(Microfacet),
    Mirror,
}

impl BSDF {
    // BSDF * cos(theta)
    #[inline(always)] pub fn eval(&self, wi: V, wo: V, uv: F2) -> Color {
        match self {
            Self::Diffuse(f) => f.eval(wi, wo, uv),
            Self::Microfacet(f) => f.eval(wi, wo),
            _ => Color::ZERO,
        }
    }

    // (color, wo, pdf, specular)
    #[inline(always)]
    pub fn sample(&self, wi: V, uv: F2, s: F2) -> (Color, V, F, bool) {
        match self {
            Self::Dielectric(f) => f.sample(wi, s),
            Self::Diffuse(f) => f.sample(uv, s),
            Self::Microfacet(f) => f.sample(wi, s),
            Self::Mirror => Mirror::sample(wi),
        }
    }

    #[inline(always)] pub fn pdf(&self, wi: V, wo: V) -> F {
        F::clamp_pos(match self {
            Self::Diffuse(_) => Diffuse::pdf(wo),
            Self::Microfacet(f) => f.pdf(wi, wo),
            _ => 0.,
        })
    }

    #[inline(always)] pub const fn is_delta(&self) -> bool
    { matches!(self, Self::Mirror | Self::Dielectric(_)) }
}

impl From<Dielectric> for BSDF
{ fn from(f: Dielectric) -> Self { Self::Dielectric(f) } }

impl From<Diffuse> for BSDF { fn from(f: Diffuse) -> Self { Self::Diffuse(f) } }

impl From<Microfacet> for BSDF
{ fn from(f: Microfacet) -> Self { Self::Microfacet(f) } }

impl From<Mirror> for BSDF { fn from(_: Mirror) -> Self { Self::Mirror } }

impl Zero for BSDF { const ZERO: Self = Self::Diffuse(Diffuse::ZERO); }

impl Default for BSDF { fn default() -> Self { Self::ZERO } }
