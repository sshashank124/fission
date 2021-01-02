use super::*;

pub struct Mirror;

impl Mirror {
    #[inline] pub fn sample(wi: V) -> (Color, V, F, bool)
    { (Color::ONE, V::from(Frame::reflect(wi)), 1., true) }
}
