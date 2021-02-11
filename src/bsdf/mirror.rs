#[allow(clippy::wildcard_imports)]
use graphite::*;

use crate::color::Color;
use crate::util::pdf::Pdf;

#[inline] pub fn sample(wi: V) -> (Pdf<Color>, V, bool)
{ (Pdf::sole(Color::ONE), Frame::reflect(wi).conv(), true) }
