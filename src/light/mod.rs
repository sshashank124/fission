mod emitter;
mod infinite;
mod point;

use std::sync::Arc;

#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

use crate::color::Color;
use crate::shape::{Shape, intersection::Its};
use crate::util::pdf::PDF;

use infinite::Infinite;
use point::Point;

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
    #[inline] pub fn sample<'a>(&'a self, its: &Its<'a>, s: F2) -> (PDF<Color>, R) {
        match self {
            Self::Area(l) => l.sample(its, s),
            Self::Infinite(l) => l.sample(its, s),
            Self::Point(l) => l.sample(its),
        }
    }

    #[inline] pub const fn is_env_light(&self) -> bool
    { matches!(self, Self::Infinite(_)) }

    #[inline] pub fn eval_env(&self, ray: &R) -> Color {
        match self {
            Self::Infinite(l) => l.eval_env(ray),
            _ => Color::ZERO,
        }
    }

    #[inline] pub fn power(&self) -> F {
        match self {
            Self::Area(l) => l.power(),
            Self::Infinite(l) => l.power(),
            Self::Point(l) => l.power(),
        }
    }
}

impl From<Arc<Shape>> for Light
{ #[inline] fn from(s: Arc<Shape>) -> Self { Self::Area(s) } }

impl From<Infinite> for Light
{ #[inline] fn from(l: Infinite) -> Self { Self::Infinite(l) } }

impl From<Point> for Light
{ #[inline] fn from(l: Point) -> Self { Self::Point(l) } }
