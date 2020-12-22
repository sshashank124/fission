mod emitter;
mod infinite;
mod point;

use crate::prelude::*;
use crate::shape::*;
use crate::texture::*;

pub use emitter::*;
pub use infinite::Infinite;
pub use point::Point;

pub enum Light {
    Area(Arc<Shape>),
    Infinite(Infinite),
    Point(Point),
}

impl Light {
    #[inline(always)] pub fn sample(&self, its: &Its, s: F2) -> (Color, R, F) {
        match self {
            Light::Area(l) => l.sample(its, s),
            Light::Infinite(l) => l.sample(its, s),
            Light::Point(l) => l.sample(its),
        }
    }

    #[inline(always)] pub fn is_env_light(&self) -> bool {
        match self {
            Light::Infinite(l) => l.is_env_light(),
            _ => false,
        }
    }

    #[inline(always)] pub fn eval_env(&self, ray: &R) -> Color {
        match self {
            Light::Infinite(l) => l.eval_env(ray),
            _ => Color::ZERO,
        }
    }

    #[inline(always)] pub fn power(&self) -> F { 1.  }
}

impl From<Arc<Shape>> for Light
{ #[inline(always)] fn from(s: Arc<Shape>) -> Self { Self::Area(s) } }

impl From<Infinite> for Light
{ #[inline(always)] fn from(s: Infinite) -> Self { Self::Infinite(s) } }

impl From<Point> for Light
{ #[inline(always)] fn from(s: Point) -> Self { Self::Point(s) } }
