use super::*;


pub struct Mirror;

impl Mirror { #[inline(always)] pub const fn new() -> Self { Self } }

impl Bxdf for Mirror {
    #[inline(always)] fn eval(&self, _: V, _: V, _: F2) -> Color
    { Color::ZERO }

    #[inline(always)] fn sample(&self, wi: V, _: F2, _: F2) -> (Color, V, F)
    { (Color::ONE, V(A3(-wi[X], -wi[Y], wi[Z])), 1.) }

    #[inline(always)] fn pdf(&self, _: V, _: V) -> F { 0. }
}
