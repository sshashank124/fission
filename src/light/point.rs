use super::*;

pub struct Point {
    intensity: Color,
    pos:       P,
}

impl Point {
    pub fn new(power: Color, pos: P) -> Self {
        Self { intensity: power * F::INV_4PI, pos }
    }

    #[inline(always)]
    pub fn sample(&self, its: &Its) -> (Color, R, F) {
        let sray = R::p2(its.p, self.pos);
        (self.intensity / sray.t.sq(), sray, 1.)
    }
}
