mod checkerboard;
mod constant;

use crate::geometry::*;

pub use checkerboard::Checkerboard;
pub use constant::Constant;


pub trait Texture<A> {
    fn eval(&self, s: F2) -> A;
}

pub enum Tex<A: Copy> {
    Checkerboard(Checkerboard<A>),
    Constant(Constant<A>),
}

impl<A: Copy> Texture<A> for Tex<A> {
    #[inline(always)] fn eval(&self, s: F2) -> A {
        match self {
            Self::Checkerboard(t) => t.eval(s),
            Self::Constant(t) => t.eval(s),
        }
    }
}

impl<A: Copy> From<Checkerboard<A>> for Tex<A> {
    #[inline(always)] fn from(f: Checkerboard<A>) -> Self
    { Self::Checkerboard(f) }
}

impl<A: Copy> From<Constant<A>> for Tex<A>
{ #[inline(always)] fn from(f: Constant<A>) -> Self { Self::Constant(f) } }

impl<A: Copy + One> Zero for Tex<A>
{ const ZERO: Self = Self::Constant(Constant::ONE); }
