use std::ops::Deref;

use crate::aggregate::*;
use crate::camera::Camera;


pub struct Scene {
    objs: Mesh,
    pub camera: Camera,
}

impl Scene {
    #[inline(always)]
    pub fn new(camera: Camera, objs: Mesh) -> Scene {
        Scene { objs, camera }
    }
}

impl Deref for Scene {
    type Target = Mesh;
    #[inline(always)]
    fn deref(&self) -> &Mesh { &self.objs }
}
