mod infinite;
mod point;

use std::ops::Deref;

use crate::geometry::*;
use crate::shape::*;

pub use infinite::Infinite;
pub use point::Point;


pub trait Lighting {
    #[inline(always)] fn eval(&self, _: &R, _: Option<F2>) -> Color
    { Color::BLACK }

    fn sample(&self, its: &Its, s: F2) -> (Color, R);
    fn pdf(&self, its: &Its, sray: &R) -> F;
}

pub struct Light {
    light: LightType,
}

pub enum LightType {
    Area(Arc<Shape>),
    Infinite(Infinite),
    Point(Point),
}

impl Light {
    #[inline(always)] pub fn new(light: LightType) -> Self
    { Self { light } }
}

impl Deref for Light { type Target = LightType;
    #[inline(always)] fn deref(&self) -> &Self::Target { &self.light }
}

impl Lighting for LightType {
    #[inline(always)] fn eval(&self, ray: &R, uv: Option<F2>) -> Color {
        match self {
            LightType::Area(l) => l.eval(ray, uv),
            LightType::Infinite(l) => l.eval(ray, uv),
            LightType::Point(l) => l.eval(ray, uv),
        }
    }

    #[inline(always)] fn sample(&self, its: &Its, s: F2) -> (Color, R) {
        match self {
            LightType::Area(l) => l.sample(its, s),
            LightType::Infinite(l) => l.sample(its, s),
            LightType::Point(l) => l.sample(its, s),
        }
    }

    #[inline(always)] fn pdf(&self, its: &Its, sray: &R) -> F {
        match self {
            LightType::Area(l) => l.pdf(its, sray),
            LightType::Infinite(l) => l.pdf(its, sray),
            LightType::Point(l) => l.pdf(its, sray),
        }
    }
}

impl From<Arc<Shape>> for LightType
{ #[inline(always)] fn from(s: Arc<Shape>) -> Self { Self::Area(s) } }

impl From<Infinite> for LightType
{ #[inline(always)] fn from(s: Infinite) -> Self { Self::Infinite(s) } }

impl From<Point> for LightType
{ #[inline(always)] fn from(s: Point) -> Self { Self::Point(s) } }
