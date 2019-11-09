use super::*;


pub struct Normals;

impl Normals {
    #[inline(always)]
    pub fn new() -> Normals {
        Normals { }
    }
}

impl Integrator for Normals {
    #[inline(always)]
    fn sample(&self, scene: &Scene, ray: R) -> Color {
        match scene.intersect(ray) {
            None => Color::BLACK,
            Some(its) => {
                let n = its.n.unit();
                Color::rgb(n.x().abs(), n.y().abs(), n.z().abs())
            }
        }
    }
}
