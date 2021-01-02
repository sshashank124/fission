mod emitter;
mod infinite;
mod point;

use crate::prelude::*;
use crate::shape::*;
use crate::texture::*;

pub use emitter::*;
pub use infinite::Infinite;
pub use point::Point;

#[derive(Debug, Deserialize)]
#[serde(tag="type")]
pub enum Light {
    #[serde(skip)] Area(Arc<Shape>),
    #[serde(rename="infinitelight")]
    Infinite(Infinite),
    #[serde(rename="pointlight")]
    Point(Point),
}

impl Light {
    #[inline(always)] pub fn sample(&self, its: &Its, s: F2) -> (Color, R, F) {
        match self {
            Self::Area(l) => l.sample(its, s),
            Self::Infinite(l) => l.sample(its, s),
            Self::Point(l) => l.sample(its),
        }
    }

    #[inline(always)] pub const fn is_env_light(&self) -> bool
    { matches!(self, Self::Infinite(_)) }

    #[inline(always)] pub fn eval_env(&self, ray: &R) -> Color {
        match self {
            Self::Infinite(l) => l.eval_env(ray),
            _ => Color::ZERO,
        }
    }

    #[inline(always)] pub const fn power() -> F { 1. }
}

impl From<Arc<Shape>> for Light
{ #[inline(always)] fn from(s: Arc<Shape>) -> Self { Self::Area(s) } }

impl From<Infinite> for Light
{ #[inline(always)] fn from(s: Infinite) -> Self { Self::Infinite(s) } }

impl From<Point> for Light
{ #[inline(always)] fn from(s: Point) -> Self { Self::Point(s) } }
