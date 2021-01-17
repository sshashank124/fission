#[allow(clippy::wildcard_imports)]
use graphite::*;

use crate::color::Color;
use crate::shape::{Intersectable, Shape, intersection::Its};
use crate::util::pdf::PDF;

impl Shape {
    #[inline] pub fn eval(&self, uv: F2) -> Color
    { self.emission.as_ref().map_or(Color::ZERO, |e| e.eval(uv)) }

    #[inline] pub fn sample(&'static self, its: &Its, s: F2) -> (PDF<Color>, R) {
        if let Some(emission) = &self.emission {
            let surface = self.sample_surface(s);
            let sray = R::p2(its.p, surface.p);
            let p = self.pdf(&surface, &sray);
            let color = if p <= 0. { Color::ZERO }
                        else { emission.eval(surface.uv) / p };
            (PDF::new(color, p), sray)
        } else { unreachable!() }
    }

    #[inline] pub fn pdf(&'static self, its: &Its, sray: &R) -> F {
        let ct = F3::dot(its.n.conv(), (-sray.d).conv());
        if ct <= 0. { 0. } else { self.surface_pdf() * sray.t.sq() / ct }
    }

    #[inline] pub fn power(&'static self) -> F
    { self.emission.as_ref().map_or(0., |e| (e.mean() * F::PI * self.surface_area()).luminance()) }
}
