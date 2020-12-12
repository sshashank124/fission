use super::*;

pub struct Normals;

impl Normals {
    pub fn new() -> Self { Self }

    #[inline(always)]
    pub fn trace(&self, scene: &Scene, ray: R) -> Color {
        scene.intersect(ray)
             .map(|its| Color(its.n.map(F::abs)))
             .unwrap_or(Color::BLACK)
    }
}
