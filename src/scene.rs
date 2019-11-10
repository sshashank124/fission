use crate::camera::Camera;
use crate::geometry::*;
use crate::structure::*;


pub struct Scene {
    pub camera: Camera,
    structure: Structure,
}

impl Scene {
    #[inline(always)]
    pub fn new(camera: Camera, structure: Structure) -> Scene {
        Scene {
            camera,
            structure,
        }
    }
}

impl Intersectable for Scene {
    #[inline(always)]
    fn bbox(&self, t: T) -> BBox {
        self.structure.bbox(t)
    }

    #[inline(always)]
    fn intersects(&self, ray: R) -> bool {
        self.structure.intersects(ray)
    }

    #[inline(always)]
    fn intersect(&self, ray: R) -> Option<Its> {
        self.structure.intersect(ray)
    }
}
