#[allow(clippy::wildcard_imports)]
use graphite::*;

#[inline] pub fn sample(wi: V) -> (Color, V, F, bool)
{ (Color::ONE, V::from(Frame::reflect(wi)), 1., true) }
