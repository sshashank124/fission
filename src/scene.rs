use std::ops::Deref;

use crate::aggregate::BVH;
use crate::camera::Camera;
use crate::geometry::*;
use crate::light::*;
use crate::shape::*;


pub struct Scene {
    shapes: BVH<Arc<Shape>>,
    pub lights: Vec<Arc<Light>>,
    env: Option<Arc<Light>>,
    pub camera: Camera,
}

impl Scene {
    #[inline(always)] pub fn new(shapes: Vec<Arc<Shape>>,
                                 lights: Vec<Light>,
                                 camera: Camera) -> Self {
        let shapes = BVH::new(shapes);
        let lights = lights.into_iter().map(Arc::new).collect::<Vec<_>>();
        let env = lights.iter().find(|light| light.is_env_light())
                               .map(Arc::clone);
        Self { shapes, lights, env, camera }
    }

    #[inline(always)] pub fn random_light(&self, s: F) -> &Light
    { &self.lights[F::discrete(s, self.lights.len() as I) as usize] }

    #[inline(always)] pub fn lenv(&self, ray: &R) -> Color
    { self.env.as_ref().map(|light| light.eval_env(ray))
                       .unwrap_or(Color::BLACK) }
}

impl Deref for Scene { type Target = BVH<Arc<Shape>>;
    #[inline(always)] fn deref(&self) -> &Self::Target { &self.shapes }
}
