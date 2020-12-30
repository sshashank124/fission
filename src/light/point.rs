use super::*;

#[derive(Debug, Deserialize)]
pub struct Point {
    #[serde(rename="power", deserialize_with="de_intensity")]
    intensity: Color,
    position:  P,
}

impl Point {
    #[inline(always)] pub fn sample(&self, its: &Its) -> (Color, R, F) {
        let sray = R::p2(its.p, self.position);
        (self.intensity / sray.t.sq(), sray, 1.)
    }
}

fn de_intensity<'de, D>(de: D) -> Result<Color, D::Error>
where D: serde::Deserializer<'de>
{ Color::deserialize(de).map(|power| power * F::INV_4PI) }
