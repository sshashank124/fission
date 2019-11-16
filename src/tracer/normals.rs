use super::*;


pub struct Normals;

impl Normals {
    #[inline(always)]
    pub fn new() -> Normals {
        Normals { }
    }
}

impl Trace for Normals {
    #[inline(always)]
    fn trace(&self, scene: &Scene, _: &mut Sampler, ray: R) -> Color {
        match scene.intersect(ray) {
            None => Color::BLACK,
            Some(its) => {
                let n = its.n.unitn();
                Color::rgb(n[X].abs(), n[Y].abs(), n[Z].abs())
            }
        }
    }
}
