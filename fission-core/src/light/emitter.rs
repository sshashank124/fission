#[allow(clippy::wildcard_imports)]
use graphite::*;

use crate::shape::{Intersectable, Shape, intersection::Its};

impl Shape {
    #[inline] pub fn eval(&self, uv: F2) -> Color
    { self.emission.as_ref().map_or(Color::ZERO, |e| e.eval(uv)) }

    #[inline] pub fn sample(&self, its: &Its, s: F2) -> (Color, R, F) {
        if let Some(emission) = &self.emission {
            let surface = self.sample_surface(s);
            let sray = R::p2(its.p, surface.p);
            let p = self.pdf(&surface, &sray);
            let color = if p <= 0. { Color::ZERO }
                        else { emission.eval(surface.uv) / p };
            (color, sray, p)
        } else { unreachable!() }
    }

    #[inline] pub fn pdf(&self, its: &Its, sray: &R) -> F {
        let ct = F3::dot(its.n, -sray.d);
        if ct <= 0. { 0. } else { self.surface_pdf() * sray.t.sq() / ct }
    }
}
