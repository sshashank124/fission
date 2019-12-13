use super::*;


pub struct Normals;

impl Normals { pub fn new() -> Self { Self } }

impl Trace for Normals {
    #[inline(always)]
    fn trace(&self, scene: &Scene, _: Sampler, ray: R) -> Color {
        scene.intersect(ray).map(|its| Color(its.n.map(F::abs)))
                            .unwrap_or(Color::BLACK)
    }
}
