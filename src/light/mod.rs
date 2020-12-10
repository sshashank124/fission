mod emitter;
mod infinite;
mod point;

use crate::prelude::*;
use crate::shape::*;
use crate::texture::*;

pub use emitter::*;
pub use infinite::Infinite;
pub use point::Point;

pub trait Lighting {
    fn eval(&self, uv: F2) -> Color;
    fn sample(&self, its: &Its, s: F2) -> (Color, R, F);
    fn pdf(&self, its: &Its, sray: &R) -> F;

    fn is_env_light(&self) -> bool { false }
    #[inline(always)]
    fn eval_env(&self, _ray: &R) -> Color { Color::ZERO }
}

pub enum Light {
    Area(Arc<Shape>),
    Infinite(Infinite),
    Point(Point),
}

impl Lighting for Light {
    #[inline(always)]
    fn eval(&self, uv: F2) -> Color {
        match self {
            Light::Area(l) => l.eval(uv),
            Light::Infinite(l) => l.eval(uv),
            Light::Point(l) => l.eval(uv),
        }
    }

    #[inline(always)]
    fn sample(&self, its: &Its, s: F2) -> (Color, R, F) {
        match self {
            Light::Area(l) => l.sample(its, s),
            Light::Infinite(l) => l.sample(its, s),
            Light::Point(l) => l.sample(its, s),
        }
    }

    #[inline(always)]
    fn pdf(&self, its: &Its, sray: &R) -> F {
        match self {
            Light::Area(l) => l.pdf(its, sray),
            Light::Infinite(l) => l.pdf(its, sray),
            Light::Point(l) => l.pdf(its, sray),
        }
    }

    fn is_env_light(&self) -> bool {
        match self {
            Light::Area(l) => l.is_env_light(),
            Light::Infinite(l) => l.is_env_light(),
            Light::Point(l) => l.is_env_light(),
        }
    }

    #[inline(always)]
    fn eval_env(&self, ray: &R) -> Color {
        match self {
            Light::Area(l) => l.eval_env(ray),
            Light::Infinite(l) => l.eval_env(ray),
            Light::Point(l) => l.eval_env(ray),
        }
    }
}

impl From<Arc<Shape>> for Light {
    fn from(s: Arc<Shape>) -> Self { Self::Area(s) }
}

impl From<Infinite> for Light {
    fn from(s: Infinite) -> Self { Self::Infinite(s) }
}

impl From<Point> for Light {
    fn from(s: Point) -> Self { Self::Point(s) }
}
