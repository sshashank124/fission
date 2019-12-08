use super::*;


pub struct Point {
    intensity: Color,
    pos: P,
}

impl Point {
    pub fn new(power: Color, pos: P) -> Self
    { Self { intensity: power * F::INV_4PI, pos } }
}

impl Lighting for Point {
    #[inline(always)] fn eval(&self, _: F2) -> Color { Color::BLACK }

    #[inline(always)] fn sample(&self, its: &Its, _: F2) -> (Color, R, F)
    { let sray = R::p2(its.p, self.pos);
      (self.intensity / sray.t.sq(), sray, 1.) }

    #[inline(always)] fn pdf(&self, _: &Its, _: &R) -> F { 0. }
}
