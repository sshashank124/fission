mod sphere;
mod triangle;

use crate::aggregate::*;
use crate::geometry::*;

pub use sphere::*;
pub use triangle::*;


pub trait Intersectable {
    fn bbox(&self) -> BBox;

    fn intersects(&self, ray: R) -> bool;
    fn intersect(&self, ray: R) -> Option<Its>;
    fn hit_info<'a>(&'a self, its: Its<'a>) -> Its<'a>;

    fn intersection_cost(&self) -> F;
}

pub struct Shape {
    shape: ShapeType,
}

pub enum ShapeType {
    BVH(BVH<Shape>),
    Mesh(Mesh),
    Sphere(Sphere),
}

impl Shape
{ #[inline(always)] pub fn new(shape: ShapeType) -> Self { Self { shape } } }

impl Intersectable for Shape {
    #[inline(always)] fn bbox(&self) -> BBox { self.shape.bbox() }

    #[inline(always)] fn intersects(&self, ray: R) -> bool
    { self.shape.intersects(ray) }

    #[inline(always)] fn intersect(&self, ray: R) -> Option<Its> {
        self.shape.intersect(ray)
                  .map(|its| its.for_shape(self))
    }

    #[inline(always)] fn hit_info<'a>(&'a self, its: Its<'a>) -> Its<'a>
    { self.shape.hit_info(its) }

    #[inline(always)] fn intersection_cost(&self) -> F
    { self.shape.intersection_cost() }
}

impl Intersectable for ShapeType {
    #[inline(always)] fn bbox(&self) -> BBox {
        match self {
            Self::BVH(s) => s.bbox(),
            Self::Mesh(s) => s.bbox(),
            Self::Sphere(s) => s.bbox(),
        }
    }

    #[inline(always)] fn intersects(&self, ray: R) -> bool {
        match self {
            Self::BVH(s) => s.intersects(ray),
            Self::Mesh(s) => s.intersects(ray),
            Self::Sphere(s) => s.intersects(ray),
        }
    }

    #[inline(always)] fn intersect(&self, ray: R) -> Option<Its> {
        match self {
            Self::BVH(s) => s.intersect(ray),
            Self::Mesh(s) => s.intersect(ray),
            Self::Sphere(s) => s.intersect(ray),
        }
    }

    #[inline(always)] fn hit_info<'a>(&'a self, its: Its<'a>) -> Its<'a> {
        match self {
            Self::BVH(s) => s.hit_info(its),
            Self::Mesh(s) => s.hit_info(its),
            Self::Sphere(s) => s.hit_info(its),
        }
    }

    #[inline(always)] fn intersection_cost(&self) -> F {
        match self {
            Self::BVH(s) => s.intersection_cost(),
            Self::Mesh(s) => s.intersection_cost(),
            Self::Sphere(s) => s.intersection_cost(),
        }
    }
}

impl From<BVH<Shape>> for ShapeType
{ #[inline(always)] fn from(s: BVH<Shape>) -> Self { Self::BVH(s) } }

impl From<Mesh> for ShapeType
{ #[inline(always)] fn from(s: Mesh) -> Self { Self::Mesh(s) } }

impl From<Sphere> for ShapeType
{ #[inline(always)] fn from(s: Sphere) -> Self { Self::Sphere(s) } }
