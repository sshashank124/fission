mod checkerboard;
mod constant;
mod gradient;
mod grid;

use std::ops::{Add, Mul};

use crate::prelude::*;

pub use checkerboard::Checkerboard;
pub use constant::Constant;
pub use gradient::Gradient;
pub use grid::Grid;

pub enum Tex<A>
{
    Checkerboard(Checkerboard<A>),
    Constant(Constant<A>),
    LinearGradient(Gradient<A, LinearScale>),
    SmoothGradient(Gradient<A, SmoothScale>),
    Grid(Grid<A>),
}

impl<A> Tex<A>
    where A: Copy + Add<Output = A> + Mul<F, Output = A>
{
    #[inline(always)]
    pub fn eval(&self, s: F2) -> A {
        match self {
            Self::Checkerboard(t) => t.eval(s),
            Self::Constant(t) => t.eval(),
            Self::LinearGradient(t) => t.eval(s),
            Self::SmoothGradient(t) => t.eval(s),
            Self::Grid(t) => t.eval(s),
        }
    }
}

impl<A> From<Checkerboard<A>> for Tex<A>
    where A: Copy + Add<Output = A> + Mul<F, Output = A>
{
    fn from(t: Checkerboard<A>) -> Self { Self::Checkerboard(t) }
}

impl<A> From<Constant<A>> for Tex<A>
    where A: Copy + Add<Output = A> + Mul<F, Output = A>
{
    fn from(t: Constant<A>) -> Self { Self::Constant(t) }
}

impl<A> From<Gradient<A, LinearScale>> for Tex<A>
    where A: Copy + Add<Output = A> + Mul<F, Output = A>
{
    fn from(t: Gradient<A, LinearScale>) -> Self { Self::LinearGradient(t) }
}

impl<A> From<Gradient<A, SmoothScale>> for Tex<A>
    where A: Copy + Add<Output = A> + Mul<F, Output = A>
{
    fn from(t: Gradient<A, SmoothScale>) -> Self { Self::SmoothGradient(t) }
}

impl<A> From<Grid<A>> for Tex<A>
    where A: Copy + Add<Output = A> + Mul<F, Output = A>
{
    fn from(t: Grid<A>) -> Self { Self::Grid(t) }
}

impl<A: Copy + Zero> Zero for Tex<A>
    where A: Add<Output = A> + Mul<F, Output = A>
{
    const ZERO: Self = Self::Constant(Constant::ZERO);
}
