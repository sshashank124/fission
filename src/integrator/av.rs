use super::*;


pub struct AverageVisibility;

impl AverageVisibility {
    #[inline(always)]
    pub fn new() -> AverageVisibility {
        AverageVisibility { }
    }
}

impl Integrator for AverageVisibility {
    #[inline(always)]
    fn sample(&self, scene: &Scene, ray: R) -> Color {
        match scene.intersect(ray) {
            None => Color::WHITE,
            Some(its) => {
                let n = its.n.unit();
                Color::rgb(n.x().abs(), n.y().abs(), n.z().abs())
            }
        }
    }
}
