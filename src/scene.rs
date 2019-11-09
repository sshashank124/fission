use crate::camera::Camera;
use crate::geometry::*;
use crate::structure::*;


pub struct Scene {
    pub camera: Camera,
    bbox: BBox,
    structure: Structure,
}

impl Scene {
    #[inline(always)]
    pub fn new(camera: Camera, structure: Structure) -> Scene {
        Scene {
            camera,
            bbox: structure.bbox(T::I),
            structure,
        }
    }
}

impl Intersectable for Scene {
    #[inline(always)]
    fn bbox(&self, t: T) -> BBox {
        t * self.bbox
    }

    #[inline(always)]
    fn intersects(&self, ray: R) -> bool {
        self.bbox.intersects(ray) && self.structure.intersects(ray)
    }

    #[inline(always)]
    fn intersect(&self, ray: R) -> Option<Its> {
        if self.bbox.intersects(ray) {
            self.structure.intersect(ray)
        } else {
            None
        }
    }
}
