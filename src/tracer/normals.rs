use super::*;


pub struct Normals;

impl Normals { pub fn new() -> Self { Self } }

impl Trace for Normals {
    #[inline(always)]
    fn trace(&self, scene: &Scene, _: &mut Sampler, ray: R) -> Color {
        match scene.intersect(ray) {
            None => Color::BLACK,
            Some(its) => Color(its.n.map(F::abs)),
        }
    }
}
