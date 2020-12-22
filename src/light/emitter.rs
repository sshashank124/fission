use super::*;

impl Shape {
    #[inline(always)] pub fn eval(&self, uv: F2) -> Color
    { self.emission.as_ref().map(|e| e.eval(uv)).unwrap_or(Color::ZERO) }

    #[inline(always)] pub fn sample(&self, its: &Its, s: F2) -> (Color, R, F) {
        if let Some(emission) = &self.emission {
            let surface = self.sample_surface(s);
            let sray = R::p2(its.p, surface.p);
            let p = self.pdf(&surface, &sray);
            let color = if p <= 0. { Color::ZERO }
                        else { emission.eval(surface.uv) / p };
            (color, sray, p)
        } else { unreachable!() }
    }

    #[inline(always)] pub fn pdf(&self, its: &Its, sray: &R) -> F {
        let ct = F3::dot(its.n, -sray.d);
        if ct <= 0. { 0.  } else { self.surface_pdf() * sray.t.sq() / ct }
    }
}
