use super::*;


pub struct Point {
    intensity: Color,
    pos: P,
    medium: Medium,
}

impl Point {
    pub fn new(power: Color, pos: P, medium: Medium) -> Self
    { Self { intensity: power * F::INV_4PI, pos, medium } }
}

impl Lighting for Point {
    #[inline(always)] fn eval(&self, _: F2) -> Color { Color::BLACK }

    #[inline(always)] fn sample(&self, its: &Its, _: F2) -> (Color, R, F)
    { let sray = R::p2(its.p, self.pos).in_medium(&self.medium);
      (self.intensity / sray.t.sq(), sray, 1.) }

    #[inline(always)] fn pdf(&self, _: &Its, _: &R) -> F { 0. }
}
