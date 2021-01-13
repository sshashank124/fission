#[allow(clippy::wildcard_imports)]
use graphite::*;

use crate::color::Color;
use crate::util::pdf::PDF;

#[inline] pub fn sample(wi: V) -> (PDF<Color>, V, bool)
{ (PDF::sole(Color::ONE), V::from(Frame::reflect(wi)), true) }
