use super::*;


impl Lighting for Shape {
    #[inline(always)] fn eval(&self, uv: F2) -> Color
    { self.emission.as_ref().map(|e| e.eval(uv)).unwrap_or(Color::ZERO) }

    #[inline(always)] fn sample(&self, its: &Its, s: F2) -> (Color, R, F) {
        if let Some(emission) = &self.emission {
            let surface = self.sample_surface(s);
            let sray = R::p2(its.p, surface.p);
            let p = self.pdf(&surface, &sray);
            let color = if p <= 0. { Color::BLACK }
                        else { emission.eval(surface.uv) / p };
            (color, sray, p)
        } else { unreachable!() }
    }

    #[inline(always)] fn pdf(&self, its: &Its, sray: &R) -> F {
        let ct = its.n.dot(-sray.d);
        if ct <= 0. { 0. }
        else { self.surface_pdf() * sray.t.sq() / ct }
    }
}
