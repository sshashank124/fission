use std::ops::Deref;

use crate::aggregate::BVH;
use crate::camera::Camera;
use crate::geometry::*;
use crate::light::Light;
use crate::shape::*;


pub struct Scene {
    shapes: BVH<Arc<Shape>>,
    pub lights: Vec<Light>,
    pub camera: Camera,
}

impl Scene {
    #[inline(always)] pub fn new(shapes: Vec<Arc<Shape>>,
                                 lights: Vec<Light>,
                                 camera: Camera) -> Self
    { Self { shapes: BVH::new(shapes), lights, camera } }

    #[inline(always)] pub fn random_light(&self, s: F) -> &Light
    { &self.lights[F::discrete(s, self.lights.len() as I) as usize] }
}

impl Deref for Scene { type Target = BVH<Arc<Shape>>;
    #[inline(always)] fn deref(&self) -> &Self::Target { &self.shapes }
}
