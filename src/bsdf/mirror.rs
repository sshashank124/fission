use super::*;

#[derive(Debug, Deserialize)]
pub struct Mirror;

impl Mirror {
    #[inline(always)] pub fn sample(&self, wi: V) -> (Color, V, F, bool)
    { (Color::ONE, V::from(Frame::reflect(wi)), 1., self.is_delta()) }

    #[inline(always)] pub fn is_delta(&self) -> bool { true }
}
