use super::*;

pub struct Mirror;

impl Mirror {
    #[inline(always)] pub fn sample(wi: V) -> (Color, V, F, bool)
    { (Color::ONE, V::from(Frame::reflect(wi)), 1., true) }
}
