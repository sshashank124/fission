mod homogeneous;
mod interface;
mod phase;

use crate::geometry::*;
use crate::sampler::*;

pub use homogeneous::Homogeneous;
pub use interface::MediumInterface;
pub use phase::*;


pub trait MediumT {
    fn tr(&self, ray: &R) -> Color;
    fn sample(&self, ray: &R, sampler: &mut Sampler) -> (Color, Option<Its>);
}

#[derive(Debug, PartialEq)]
pub enum Medium {
    Vacuum,
    Homogeneous(Homogeneous),
}

impl MediumT for Medium {
    #[inline(always)] fn tr(&self, ray: &R) -> Color {
        match self {
            Self::Vacuum => Color::ONE,
            Self::Homogeneous(m) => m.tr(ray),
        }
    }

    #[inline(always)]
    fn sample(&self, ray: &R, sampler: &mut Sampler) -> (Color, Option<Its>) {
        let (color, its) = match self {
            Self::Vacuum => (Color::ONE, None),
            Self::Homogeneous(m) => m.sample(ray, sampler),
        };

        let medium_interaction = if let Some(its) = &its {
            its.phase.is_some()
        } else { false };

        if medium_interaction {
            return (color, its.map(|its| its.in_medium(self)))
        }

        (color, its)
    }
}

impl From<Homogeneous> for Medium
{ fn from(m: Homogeneous) -> Self { Self::Homogeneous(m) } }

impl Zero for Medium { const ZERO: Self = Self::Vacuum; }
