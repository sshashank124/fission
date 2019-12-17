use super::*;


pub struct MediumInterface(A2<Medium>);

impl MediumInterface {
    pub const fn new(outside: Medium, inside: Medium) -> Self
    { Self(A2(outside, inside)) }

    #[inline(always)] pub fn towards(&self, ct: F) -> &Medium
    { &self.0[ct < 0.] }
}

impl Zero for MediumInterface
{ const ZERO: Self = Self::new(Medium::ZERO, Medium::ZERO); }
