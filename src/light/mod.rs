mod infinite;
mod point;

use std::ops::Deref;

use crate::geometry::*;

pub use infinite::Infinite;
pub use point::Point;


pub trait Lighting {
    #[inline(always)] fn le(&self, _: &R) -> Color { Color::BLACK }
    fn sample(&self, pos: P, u: F2) -> (Color, R);
}

pub struct Light {
    light: LightType,
}

pub enum LightType {
    Infinite(Infinite),
    Point(Point),
}

impl Light
{ #[inline(always)] pub fn new(light: LightType) -> Self { Self { light } } }

impl Deref for Light { type Target = LightType;
    #[inline(always)] fn deref(&self) -> &Self::Target { &self.light }
}

impl Lighting for LightType {
    #[inline(always)] fn le(&self, ray: &R) -> Color {
        match self {
            LightType::Infinite(l) => l.le(ray),
            LightType::Point(l) => l.le(ray),
        }
    }

    #[inline(always)] fn sample(&self, pos: P, u: F2) -> (Color, R) {
        match self {
            LightType::Infinite(l) => l.sample(pos, u),
            LightType::Point(l) => l.sample(pos, u),
        }
    }
}

impl From<Infinite> for LightType
{ #[inline(always)] fn from(s: Infinite) -> Self { Self::Infinite(s) } }

impl From<Point> for LightType
{ #[inline(always)] fn from(s: Point) -> Self { Self::Point(s) } }
