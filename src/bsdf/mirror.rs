use super::*;

pub struct Mirror;

impl Mirror {
    pub const fn new() -> Self { Self }

    #[inline(always)]
    pub fn sample(&self, wi: V) -> (Color, V, F, bool) {
        (Color::ONE, Frame::reflect(wi), 1., self.is_delta())
    }

    #[inline(always)]
    pub fn is_delta(&self) -> bool { true }
}
