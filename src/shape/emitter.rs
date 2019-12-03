use super::*;

use crate::light::*;


impl Lighting for Shape {
    #[inline(always)] fn eval(&self, _: &R, uv: Option<F2>) -> Color
    { uv.and_then(|uv| self.emission.as_ref().map(|e| e.eval(uv)))
        .unwrap_or(Color::BLACK) }

    #[inline(always)] fn sample(&self, its: &Its, s: F2) -> (Color, R) {
        if let Some(emission) = &self.emission {
            let surface = self.sample_surface(s);
            let sray = R::p2(its.p, surface.p);
            let p = self.pdf(&surface, &sray);
            let color = if p <= 0. { Color::BLACK }
                        else { emission.eval(surface.uv) / p };
            (color, sray)
        } else { unreachable!() }
    }

    #[inline(always)] fn pdf(&self, its: &Its, sray: &R) -> F
    { self.surface_pdf() * sray.t.sq() / its.n.dot(-sray.d) }
}
