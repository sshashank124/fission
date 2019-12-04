mod emitter;
mod infinite;
mod point;

use crate::geometry::*;
use crate::shape::*;
use crate::texture::*;

pub use emitter::*;
pub use infinite::Infinite;
pub use point::Point;


pub trait Lighting {
    fn eval(&self, ray: &R, uv: Option<F2>) -> Color;
    fn sample(&self, its: &Its, s: F2) -> (Color, R, F);
    fn pdf(&self, its: &Its, sray: &R) -> F;
}

pub enum Light {
    Area(Arc<Shape>),
    Infinite(Infinite),
    Point(Point),
}

impl Lighting for Light {
    #[inline(always)] fn eval(&self, ray: &R, uv: Option<F2>) -> Color {
        match self {
            Light::Area(l) => l.eval(ray, uv),
            Light::Infinite(l) => l.eval(ray, uv),
            Light::Point(l) => l.eval(ray, uv),
        }
    }

    #[inline(always)] fn sample(&self, its: &Its, s: F2) -> (Color, R, F) {
        match self {
            Light::Area(l) => l.sample(its, s),
            Light::Infinite(l) => l.sample(its, s),
            Light::Point(l) => l.sample(its, s),
        }
    }

    #[inline(always)] fn pdf(&self, its: &Its, sray: &R) -> F {
        match self {
            Light::Area(l) => l.pdf(its, sray),
            Light::Infinite(l) => l.pdf(its, sray),
            Light::Point(l) => l.pdf(its, sray),
        }
    }
}

impl From<Arc<Shape>> for Light
{ #[inline(always)] fn from(s: Arc<Shape>) -> Self { Self::Area(s) } }

impl From<Infinite> for Light
{ #[inline(always)] fn from(s: Infinite) -> Self { Self::Infinite(s) } }

impl From<Point> for Light
{ #[inline(always)] fn from(s: Point) -> Self { Self::Point(s) } }
