use super::*;


pub struct Mirror;

impl Mirror { #[inline(always)] pub const fn new() -> Self { Self } }

impl Bxdf for Mirror {
    #[inline(always)] fn eval(&self, _: V, _: V, _: F2) -> Color
    { Color::BLACK }

    #[inline(always)] fn sample(&self, wi: V, _: F2, s: F2) -> (Color, V)
    { (Color::WHITE, self.sample_dir(wi, s)) }

    #[inline(always)] fn sample_dir(&self, wi: V, _: F2) -> V
    { V(A3(-wi[X], -wi[Y], wi[Z])) }

    #[inline(always)] fn pdf(&self, _: V, _: V) -> F { 0. }
}
