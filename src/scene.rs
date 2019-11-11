use crate::aggregate::*;
use crate::camera::Camera;
use crate::geometry::*;


pub struct Scene {
    pub camera: Camera,
    objs: Mesh,
}

impl Scene {
    #[inline(always)]
    pub fn new(camera: Camera, objs: Mesh) -> Scene {
        Scene {
            camera,
            objs,
        }
    }
}

impl Intersectable for Scene {
    #[inline(always)]
    fn bbox(&self, t: T) -> BBox {
        self.objs.bbox(t)
    }

    #[inline(always)]
    fn intersects(&self, ray: R) -> bool {
        self.objs.intersects(ray)
    }

    #[inline(always)]
    fn intersect(&self, ray: R) -> Option<Its> {
        self.objs.intersect(ray)
    }
}
