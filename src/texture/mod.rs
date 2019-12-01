mod checkerboard;
mod constant;
mod gradient;
mod random_grid;

use std::ops::{Add, Mul};

use crate::geometry::*;

pub use checkerboard::Checkerboard;
pub use constant::Constant;
pub use gradient::Gradient;
pub use random_grid::RandomGrid;


pub trait Texture<A> {
    fn eval(&self, s: F2) -> A;
}

pub enum Tex<A> where A: Copy
                       + Add<Output=A>
                       + Mul<F, Output=A> {
    Checkerboard(Checkerboard<A>),
    Constant(Constant<A>),
    LinearGradient(Gradient<A, LinearInterp>),
    SmoothGradient(Gradient<A, SmoothInterp>),
    RandomGrid(RandomGrid<A>),
}

impl<A> Texture<A> for Tex<A> where A: Copy
                                     + Add<Output=A>
                                     + Mul<F, Output=A> {
    #[inline(always)] fn eval(&self, s: F2) -> A {
        match self {
            Self::Checkerboard(t) => t.eval(s),
            Self::Constant(t) => t.eval(s),
            Self::LinearGradient(t) => t.eval(s),
            Self::SmoothGradient(t) => t.eval(s),
            Self::RandomGrid(t) => t.eval(s),
        }
    }
}

impl<A> From<Checkerboard<A>> for Tex<A> where A: Copy
                                                + Add<Output=A>
                                                + Mul<F, Output=A> {
    #[inline(always)] fn from(t: Checkerboard<A>) -> Self
    { Self::Checkerboard(t) }
}

impl<A> From<Constant<A>> for Tex<A> where A: Copy
                                            + Add<Output=A>
                                            + Mul<F, Output=A>
{ #[inline(always)] fn from(t: Constant<A>) -> Self { Self::Constant(t) } }

impl<A> From<Gradient<A, LinearInterp>> for Tex<A> where A: Copy
                                                          + Add<Output=A>
                                                          + Mul<F, Output=A> {
    #[inline(always)] fn from(t: Gradient<A, LinearInterp>) -> Self
    { Self::LinearGradient(t) }
}

impl<A> From<Gradient<A, SmoothInterp>> for Tex<A> where A: Copy
                                                          + Add<Output=A>
                                                          + Mul<F, Output=A> {
    #[inline(always)] fn from(t: Gradient<A, SmoothInterp>) -> Self
    { Self::SmoothGradient(t) }
}

impl<A> From<RandomGrid<A>> for Tex<A> where A: Copy
                                              + Add<Output=A>
                                              + Mul<F, Output=A>
{ #[inline(always)] fn from(t: RandomGrid<A>) -> Self { Self::RandomGrid(t) } }

impl<A: Copy + One> Zero for Tex<A> where A: Add<Output=A> + Mul<F, Output=A>
{ const ZERO: Self = Self::Constant(Constant::ONE); }
