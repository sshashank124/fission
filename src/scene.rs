use std::ops::Deref;

use crate::aggregate::BVH;
use crate::camera::Camera;
use crate::light::Light;
use crate::shape::Shape;


pub struct Scene {
    shapes: BVH<Shape>,
    pub lights: Vec<Light>,
    pub camera: Camera,
}

impl Scene {
    #[inline(always)]
    pub fn new(shapes: Vec<Shape>, lights: Vec<Light>, camera: Camera) -> Self
    { Self { shapes: BVH::new(shapes), lights, camera } }
}

impl Deref for Scene { type Target = BVH<Shape>;
    #[inline(always)] fn deref(&self) -> &Self::Target { &self.shapes }
}
