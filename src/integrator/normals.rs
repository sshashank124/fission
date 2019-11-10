use super::*;


pub struct Normals;

impl Normals {
    #[inline(always)]
    pub fn new() -> Normals {
        Normals { }
    }
}

impl Integrate for Normals {
    #[inline(always)]
    fn sample(&self, scene: &Scene, _sampler: &mut Sampler, ray: R) -> Color {
        match scene.intersect(ray) {
            None => Color::BLACK,
            Some(its) => {
                let n = its.n.unit();
                Color::rgb(n.x().abs() as f32,
                           n.y().abs() as f32,
                           n.z().abs() as f32)
            }
        }
    }
}
