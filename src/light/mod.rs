mod point;

use std::ops::Deref;

use crate::geometry::*;

pub use point::Point;


pub type LightQueryResult = (Color, R);

pub trait Lighting {
    fn eval(&self, pos: P) -> LightQueryResult;
    fn sample(&self, pos: P, u: F2) -> LightQueryResult;
}

pub struct Light {
    light: LightType,
}

pub enum LightType {
    Point(Point),
}

impl Light
{ #[inline(always)] pub fn new(light: LightType) -> Self { Self { light } } }

impl Deref for Light { type Target = LightType;
    #[inline(always)] fn deref(&self) -> &Self::Target { &self.light }
}

impl Lighting for LightType {
    #[inline(always)] fn eval(&self, pos: P) -> LightQueryResult {
        match self {
            LightType::Point(l) => l.eval(pos),
        }
    }

    #[inline(always)] fn sample(&self, pos: P, u: F2) -> LightQueryResult {
        match self {
            LightType::Point(l) => l.sample(pos, u),
        }
    }
}

impl From<Point> for LightType
{ #[inline(always)] fn from(s: Point) -> Self { Self::Point(s) } }
