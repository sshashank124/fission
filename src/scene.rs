use crate::camera::Camera;
use crate::geometry::*;
use crate::structure::*;


pub struct Scene {
    pub camera: Camera,
    bbox: BBox,
    structure: Structure,
}

impl Scene {
    #[inline]
    pub fn new(camera: Camera, structure: Structure) -> Scene {
        Scene {
            camera,
            bbox: structure.bbox(T::I),
            structure,
        }
    }
}

impl Intersectable for Scene {
    #[inline]
    fn bbox(&self, t: T) -> BBox {
        t * self.bbox
    }

    #[inline]
    fn intersects(&self, ray: R) -> bool {
        self.bbox.intersects(ray) && self.structure.intersects(ray)
    }

    #[inline]
    fn intersect(&self, ray: R) -> Option<Its> {
        if self.bbox.intersects(ray) {
            self.structure.intersect(ray)
        } else {
            None
        }
    }
}
