use super::*;

pub struct Mirror;

impl Mirror {
    pub const fn new() -> Self { Self }
}

impl BXDF for Mirror {
    #[inline(always)]
    fn eval(&self, _: V, _: V, _: F2) -> Color { Color::ZERO }

    #[inline(always)]
    fn sample(&self, wi: V, _: F2, _: F2) -> (Color, V, F, bool) {
        (Color::ONE, Frame::reflect(wi), 1., self.is_delta())
    }

    #[inline(always)]
    fn pdf(&self, _: V, _: V) -> F { 0. }

    #[inline(always)]
    fn is_delta(&self) -> bool { true }
}
