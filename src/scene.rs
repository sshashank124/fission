use std::ops::Deref;

use crate::camera::Camera;
use crate::structure::Structure;


pub struct Scene {
    objects: Structure,
    pub camera: Camera,
}

impl Scene {
    #[inline(always)]
    pub fn new(objects: Structure, camera: Camera) -> Scene {
        Scene { objects, camera }
    }
}

impl Deref for Scene {
    type Target = Structure;
    #[inline(always)] fn deref(&self) -> &Self::Target { &self.objects }
}
