mod bvh;
pub mod mesh;
mod sphere;
mod seq;

use crate::geometry::*;
pub use bvh::BVH;
pub use mesh::Mesh;
pub use sphere::Sphere;


pub enum StructureType {
    Mesh(Mesh),
    Sphere(Sphere),
    Sequence(Vec<Structure>),
    BVH(BVH<Structure>),
}

pub trait Intersectable {
    fn bbox(&self, t: T) -> BBox;
    fn intersects(&self, ray: R) -> bool;
    fn intersect(&self, ray: R) -> Option<Its>;
}

pub struct Structure {
    structure: StructureType,
    bbox: BBox,
    to_world: T,
}

impl Structure {
    #[inline(always)]
    pub fn new<S>(structure: S, to_world: T) -> Structure
            where S: Intersectable + Into<StructureType> {
        let structure = structure.into();
        Structure {
            bbox: structure.bbox(to_world),
            structure,
            to_world,
        }
    }
}

impl Intersectable for Structure {
    #[inline(always)]
    fn bbox(&self, t: T) -> BBox {
        t * self.bbox
    }

    #[inline(always)]
    fn intersects(&self, ray: R) -> bool {
        self.bbox.intersects(ray) &&
            self.structure.intersects(self.to_world / ray)
    }

    #[inline(always)]
    fn intersect(&self, ray: R) -> Option<Its> {
        if self.bbox.intersects(ray) {
            self.structure.intersect(self.to_world / ray)
                          .map(|its| self.to_world * its)
        } else {
            None
        }
    }
}

impl Intersectable for StructureType {
    #[inline(always)]
    fn bbox(&self, t: T) -> BBox {
        match self {
            StructureType::Mesh(mesh) => mesh.bbox(t),
            StructureType::Sphere(sphere) => sphere.bbox(t),
            StructureType::Sequence(sequence) => sequence.bbox(t),
            StructureType::BVH(bvh) => bvh.bbox(t),
        }
    }

    #[inline(always)]
    fn intersects(&self, ray: R) -> bool {
        match self {
            StructureType::Mesh(mesh) => mesh.intersects(ray),
            StructureType::Sphere(sphere) => sphere.intersects(ray),
            StructureType::Sequence(sequence) => sequence.intersects(ray),
            StructureType::BVH(bvh) => bvh.intersects(ray),
        }
    }

    #[inline(always)]
    fn intersect(&self, ray: R) -> Option<Its> {
        match self {
            StructureType::Mesh(mesh) => mesh.intersect(ray),
            StructureType::Sphere(sphere) => sphere.intersect(ray),
            StructureType::Sequence(sequence) => sequence.intersect(ray),
            StructureType::BVH(bvh) => bvh.intersect(ray),
        }
    }
}

impl From<Mesh> for StructureType {
    #[inline(always)]
    fn from(mesh: Mesh) -> StructureType {
        StructureType::Mesh(mesh)
    }
}

impl From<Sphere> for StructureType {
    #[inline(always)]
    fn from(sphere: Sphere) -> StructureType {
        StructureType::Sphere(sphere)
    }
}

impl From<Vec<Structure>> for StructureType {
    #[inline(always)]
    fn from(sequence: Vec<Structure>) -> StructureType {
        StructureType::Sequence(sequence)
    }
}

impl From<BVH<Structure>> for StructureType {
    #[inline(always)]
    fn from(bvh: BVH<Structure>) -> StructureType {
        StructureType::BVH(bvh)
    }
}
