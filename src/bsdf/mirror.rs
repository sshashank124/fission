use super::*;


pub struct Mirror;

impl Mirror { #[inline(always)] pub const fn new() -> Self { Self } }

impl Bxdf for Mirror {
    #[inline(always)] fn eval(&self, _: V, _: V, _: F2) -> Color
    { Color::BLACK }

    #[inline(always)] fn sample(&self, wo: V, _: F2, _: F2) -> (Color, V)
    { (Color::WHITE, V(A3(-wo[X], -wo[Y], wo[Z]))) }
}
