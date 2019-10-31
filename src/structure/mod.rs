mod bvh;
pub mod mesh;
mod sphere;
mod vec;

use crate::geometry::*;
pub use mesh::Mesh;
pub use sphere::Sphere;


pub enum StructureType {
    Mesh(Mesh),
    Sphere(Sphere),
    Sequence(Vec<Structure>),
}

pub trait Intersectable {
    fn bbox(&self, t: T) -> BBox;
    fn intersects(&self, ray: R) -> bool;
    fn intersect(&self, ray: R) -> Option<Its>;
}

pub struct Structure {
    structure: StructureType,  // use Box<StructureType> here?
    bbox: BBox,
    to_world: T,
}

impl Structure {
    #[inline]
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
    #[inline]
    fn bbox(&self, t: T) -> BBox {
        t * self.bbox
    }

    #[inline]
    fn intersects(&self, ray: R) -> bool {
        self.bbox.intersects(ray) &&
            self.structure.intersects(self.to_world / ray)
    }

    #[inline]
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
    #[inline]
    fn bbox(&self, t: T) -> BBox {
        match self {
            StructureType::Mesh(mesh) => mesh.bbox(t),
            StructureType::Sphere(sphere) => sphere.bbox(t),
            StructureType::Sequence(sequence) => sequence.bbox(t),
        }
    }

    #[inline]
    fn intersects(&self, ray: R) -> bool {
        match self {
            StructureType::Mesh(mesh) => mesh.intersects(ray),
            StructureType::Sphere(sphere) => sphere.intersects(ray),
            StructureType::Sequence(sequence) => sequence.intersects(ray),
        }
    }

    #[inline]
    fn intersect(&self, ray: R) -> Option<Its> {
        match self {
            StructureType::Mesh(mesh) => mesh.intersect(ray),
            StructureType::Sphere(sphere) => sphere.intersect(ray),
            StructureType::Sequence(sequence) => sequence.intersect(ray),
        }
    }
}

impl From<Mesh> for StructureType {
    #[inline]
    fn from(mesh: Mesh) -> StructureType {
        StructureType::Mesh(mesh)
    }
}

impl From<Sphere> for StructureType {
    #[inline]
    fn from(sphere: Sphere) -> StructureType {
        StructureType::Sphere(sphere)
    }
}

impl From<Vec<Structure>> for StructureType {
    #[inline]
    fn from(sequence: Vec<Structure>) -> StructureType {
        StructureType::Sequence(sequence)
    }
}
