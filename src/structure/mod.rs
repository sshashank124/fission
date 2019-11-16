mod triangle;

use crate::aggregate::*;
use crate::geometry::*;

pub use triangle::*;


pub trait Intersectable {
    fn bbox(&self, t: T) -> BBox;
    fn intersects(&self, ray: R) -> bool;
    fn intersect(&self, ray: R) -> Option<Its>;
    fn hit_info(&self, its: Its) -> Its;
}

pub struct Structure {
    structure: StructureType,
    to_world: T,
}

pub enum StructureType {
    BVH(BVH<Structure>),
    Mesh(Mesh),
}

impl Structure {
    #[inline(always)]
    pub fn new<S>(obj: S, to_world: T) -> Self where S: Into<StructureType> {
        Self { structure: obj.into(), to_world }
    }
}

impl Intersectable for Structure {
    #[inline(always)] fn bbox(&self, t: T) -> BBox {
        self.structure.bbox(t * self.to_world)
    }

    #[inline(always)]
    fn intersects(&self, ray: R) -> bool {
        self.structure.intersects(self.to_world / ray)
    }

    #[inline(always)]
    fn intersect(&self, ray: R) -> Option<Its> {
        self.structure.intersect(self.to_world / ray)
                      .map(|its| self.to_world * its)
    }

    #[inline(always)]
    fn hit_info(&self, its: Its) -> Its { self.structure.hit_info(its) }
}

impl Intersectable for StructureType {
    #[inline(always)]
    fn bbox(&self, t: T) -> BBox {
        match self {
            Self::BVH(s) => s.bbox(t),
            Self::Mesh(s) => s.bbox(t),
        }
    }

    #[inline(always)]
    fn intersects(&self, ray: R) -> bool {
        match self {
            Self::BVH(s) => s.intersects(ray),
            Self::Mesh(s) => s.intersects(ray),
        }
    }

    #[inline(always)]
    fn intersect(&self, ray: R) -> Option<Its> {
        match self {
            Self::BVH(s) => s.intersect(ray),
            Self::Mesh(s) => s.intersect(ray),
        }
    }

    #[inline(always)]
    fn hit_info(&self, its: Its) -> Its {
        match self {
            Self::BVH(s) => s.hit_info(its),
            Self::Mesh(s) => s.hit_info(its),
        }
    }
}

impl From<BVH<Structure>> for StructureType {
    #[inline(always)]
    fn from(s: BVH<Structure>) -> Self { Self::BVH(s) }
}

impl From<Mesh> for StructureType {
    #[inline(always)]
    fn from(s: Mesh) -> Self { Self::Mesh(s) }
}
