mod triangle;

use crate::aggregate::*;
use crate::geometry::*;

pub use triangle::*;


pub trait Intersectable {
    fn bbox(&self, t: T) -> BBox;
    fn intersects(&self, ray: R) -> bool;
    fn intersect(&self, ray: R) -> Option<Its>;
    fn hit_info(&self, its: Its) -> Its;
    fn intersection_cost(&self) -> F;
}

pub struct Shape {
    shape: ShapeType,
    to_world: T,
}

pub enum ShapeType {
    BVH(BVH<Shape>),
    Mesh(Mesh),
}

impl Shape {
    #[inline(always)] pub fn new(shape: ShapeType, to_world: T) -> Self
    { Self { shape, to_world } }
}

impl Intersectable for Shape {
    #[inline(always)] fn bbox(&self, t: T) -> BBox
    { self.shape.bbox(t * self.to_world) }

    #[inline(always)] fn intersects(&self, ray: R) -> bool
    { self.shape.intersects(self.to_world / ray) }

    #[inline(always)]
    fn intersect(&self, ray: R) -> Option<Its>
    { self.shape.intersect(self.to_world / ray)
                .map(|its| self.to_world * its) }

    #[inline(always)] fn hit_info(&self, its: Its) -> Its
    { self.shape.hit_info(its) }

    #[inline(always)] fn intersection_cost(&self) -> F
    { self.shape.intersection_cost() }
}

impl Intersectable for ShapeType {
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

    #[inline(always)]
    fn intersection_cost(&self) -> F {
        match self {
            Self::BVH(s) => s.intersection_cost(),
            Self::Mesh(s) => s.intersection_cost(),
        }
    }
}

impl From<BVH<Shape>> for ShapeType
{ #[inline(always)] fn from(s: BVH<Shape>) -> Self { Self::BVH(s) } }

impl From<Mesh> for ShapeType
{ #[inline(always)] fn from(s: Mesh) -> Self { Self::Mesh(s) } }
