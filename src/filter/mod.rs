mod square;
mod gaussian;

use crate::types::*;

pub use square::Square;
pub use gaussian::Gaussian;


pub const FILTER_RESOLUTION: usize = 32;

pub trait Filter {
    fn eval(&self, dist: F) -> F;
    fn radius(&self) -> F;
}

#[derive(Clone, Copy)]
pub enum ReconstructionFilter {
    Square(Square),
    Gaussian(Gaussian),
}

impl ReconstructionFilter {
    #[inline(always)] pub fn default() -> Self { Square::new(0.5).into() }
}

impl Filter for ReconstructionFilter {
    #[inline(always)]
    fn eval(&self, dist: F) -> F {
        match self {
            ReconstructionFilter::Square(f) => f.eval(dist),
            ReconstructionFilter::Gaussian(f) => f.eval(dist),
        }
    }

    #[inline(always)]
    fn radius(&self) -> F {
        match self {
            ReconstructionFilter::Square(f) => f.radius(),
            ReconstructionFilter::Gaussian(f) => f.radius(),
        }
    }
}

impl From<Square> for ReconstructionFilter {
    #[inline(always)] fn from(square: Square) -> Self { Self::Square(square) }
}

impl From<Gaussian> for ReconstructionFilter {
    #[inline(always)]
    fn from(gaussian: Gaussian) -> Self { Self::Gaussian(gaussian) }
}
