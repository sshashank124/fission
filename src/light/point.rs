use super::*;


pub struct Point {
    intensity: Color,
    pos: P,
}

impl Point {
    #[inline(always)] pub fn new(power: Color, pos: P) -> Self
    { Self { intensity: power * F::INV_4PI, pos } }
}

impl Lighting for Point {
    #[inline(always)] fn sample(&self, its: &Its, _: F2) -> (Color, R) {
        let sray = R::p2(its.p, self.pos);
        (self.intensity / sray.t.sq(), sray)
    }

    #[inline(always)] fn pdf(&self, _: &Its, _: &R) -> F { 0. }
}
