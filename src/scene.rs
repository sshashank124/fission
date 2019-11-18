use std::ops::Deref;

use crate::camera::Camera;
use crate::shape::Shape;


pub struct Scene {
    shapes: Shape,
    pub camera: Camera,
}

impl Scene {
    #[inline(always)] pub fn new(shapes: Shape, camera: Camera) -> Self
    { Self { shapes, camera } }
}

impl Deref for Scene { type Target = Shape;
    #[inline(always)] fn deref(&self) -> &Shape { &self.shapes }
}
