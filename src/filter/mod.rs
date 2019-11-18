mod square;
mod gaussian;

use crate::types::*;

pub use square::Square;
pub use gaussian::Gaussian;


pub trait Filter {
    fn eval(&self, dist: F) -> F;
    fn radius(&self) -> F;
}

#[derive(Clone)]
pub enum ReconstructionFilter {
    Square(Square),
    Gaussian(Gaussian),
}

impl ReconstructionFilter {
    #[inline(always)] pub fn default() -> Self { Square::new(F::HALF).into() }
}

impl Filter for ReconstructionFilter {
    #[inline(always)]
    fn eval(&self, dist: F) -> F {
        match self {
            Self::Square(f) => f.eval(dist),
            Self::Gaussian(f) => f.eval(dist),
        }
    }

    #[inline(always)]
    fn radius(&self) -> F {
        match self {
            Self::Square(f) => f.radius(),
            Self::Gaussian(f) => f.radius(),
        }
    }
}

impl From<Square> for ReconstructionFilter
{ #[inline(always)] fn from(f: Square) -> Self { Self::Square(f) } }

impl From<Gaussian> for ReconstructionFilter
{ #[inline(always)] fn from(f: Gaussian) -> Self { Self::Gaussian(f) } }
