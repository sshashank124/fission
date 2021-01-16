mod bitmap;
mod checkerboard;
mod constant;
mod gradient;

use std::ops::{Add, Mul};

#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

use checkerboard::Checkerboard;
use constant::Constant;
use gradient::Gradient;

use crate::image::bitmap::Bitmap;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag="type", rename_all="snake_case")]
pub enum Tex<A> {
    #[serde(deserialize_with="bitmap::de_from_config")]
    Bitmap(Bitmap<A>),
    Checkerboard(Checkerboard<A>),
    Constant(Constant<A>),
    LinearGradient(Gradient<A, LinearScale>),
    SmoothGradient(Gradient<A, SmoothScale>),
}

impl<A> Tex<A> where A: Copy + Zero + Add<Output = A> + Mul<F, Output = A> {
    #[inline] pub fn eval(&self, s: F2) -> A {
        match self {
            Self::Bitmap(t) => t.eval(s),
            Self::Checkerboard(t) => t.eval(s),
            Self::Constant(t) => t.eval(),
            Self::LinearGradient(t) => t.eval(s),
            Self::SmoothGradient(t) => t.eval(s),
        }
    }

    #[inline] pub fn mean(&self) -> A {
        match self {
            Self::Bitmap(t) => t.mean(),
            Self::Checkerboard(t) => t.mean(),
            Self::Constant(t) => t.mean(),
            Self::LinearGradient(t) => t.mean(),
            Self::SmoothGradient(t) => t.mean(),
        }
    }
}

impl<A> From<Bitmap<A>> for Tex<A> where A: Copy + Add<Output=A> + Mul<F, Output=A>
{ fn from(t: Bitmap<A>) -> Self { Self::Bitmap(t) } }

impl<A> From<Checkerboard<A>> for Tex<A> where A: Copy + Add<Output=A> + Mul<F, Output=A>
{ fn from(t: Checkerboard<A>) -> Self { Self::Checkerboard(t) } }

impl<A> From<Constant<A>> for Tex<A> where A: Copy + Add<Output=A> + Mul<F, Output=A>
{ fn from(t: Constant<A>) -> Self { Self::Constant(t) } }

impl<A> From<Gradient<A, LinearScale>> for Tex<A> where A: Copy + Add<Output=A> + Mul<F, Output=A>
{ fn from(t: Gradient<A, LinearScale>) -> Self { Self::LinearGradient(t) } }

impl<A> From<Gradient<A, SmoothScale>> for Tex<A> where A: Copy + Add<Output=A> + Mul<F, Output=A>
{ fn from(t: Gradient<A, SmoothScale>) -> Self { Self::SmoothGradient(t) } }

impl<A> Zero for Tex<A> where A: Zero
{ const ZERO: Self = Self::Constant(Constant::ZERO); }

impl<A> Default for Tex<A> where A: Default
{ fn default() -> Self { Self::Constant(Constant::default()) } }
