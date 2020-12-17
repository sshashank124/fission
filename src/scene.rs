use std::ops::Deref;

use crate::aggregate::BVH;
use crate::camera::Camera;
use crate::light::*;
use crate::prelude::*;
use crate::shape::*;
use crate::util::DiscretePDF;

pub struct Scene {
    shapes:      BVH<Arc<Shape>>,
    pub camera:  Camera,
    pub lights:  Vec<Arc<Light>>,
    pub lights_dpdf: DiscretePDF,
    env:         Option<Arc<Light>>,
}

impl Scene {
    pub fn new(shapes: Vec<Arc<Shape>>,
               lights: Vec<Light>,
               camera: Camera)
               -> Self {
        let shapes = BVH::new(shapes);
        let lights_dpdf = DiscretePDF::new(lights.iter(), Light::power);
        let lights = lights.into_iter().map(Arc::new).collect::<Vec<_>>();
        let env =
            lights.iter().find(|light| light.is_env_light()).map(Arc::clone);
        Self { shapes, camera, lights, lights_dpdf, env }
    }

    #[inline(always)]
    pub fn sample_random_light(&self, its: &Its, mut s: F2) -> (Color, R, F) {
        let idx = self.lights_dpdf.sample(&mut s[0]);
        self.lights[idx].sample(its, s)
    }

    #[inline(always)]
    pub fn lenv(&self, ray: &R) -> Color {
        self.env
            .as_ref()
            .map(|light| light.eval_env(ray))
            .unwrap_or(Color::BLACK)
    }
}

impl Deref for Scene {
    type Target = BVH<Arc<Shape>>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.shapes }
}
