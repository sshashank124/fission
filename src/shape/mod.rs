mod mesh;
mod sphere;

use crate::aggregate::*;
use crate::geometry::*;
use crate::bsdf::*;

pub use mesh::*;
pub use sphere::*;


pub trait Intersectable {
    fn bbox(&self) -> BBox;

    fn intersects(&self, ray: R) -> bool;
    fn intersect(&self, ray: R) -> Option<Its>;
    fn hit_info<'a>(&'a self, its: Its<'a>) -> Its<'a>;

    fn intersection_cost(&self) -> F;
}

pub struct Shape {
    pub shape: ShapeType,
    pub bsdf: Bsdf,
}

pub enum ShapeType {
    None,
    Mesh(Mesh),
    Sphere(Sphere),
}

impl Shape {
    #[inline(always)]
    pub const fn new(shape: ShapeType, bsdf: Bsdf) -> Self
    { Self { shape, bsdf } }
}

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

impl Zero for Shape
{ const ZERO: Self = Self::new(ShapeType::ZERO, Bsdf::ZERO); }

impl Intersectable for ShapeType {
    #[inline(always)] fn bbox(&self) -> BBox {
        match self {
            Self::None => BBox::ZERO,
            Self::Mesh(s) => s.bbox(),
            Self::Sphere(s) => s.bbox(),
        }
    }

    #[inline(always)] fn intersects(&self, ray: R) -> bool {
        match self {
            Self::None => false,
            Self::Mesh(s) => s.intersects(ray),
            Self::Sphere(s) => s.intersects(ray),
        }
    }

    #[inline(always)] fn intersect(&self, ray: R) -> Option<Its> {
        match self {
            Self::None => None,
            Self::Mesh(s) => s.intersect(ray),
            Self::Sphere(s) => s.intersect(ray),
        }
    }

    #[inline(always)] fn hit_info<'a>(&'a self, its: Its<'a>) -> Its<'a> {
        match self {
            Self::None => its,
            Self::Mesh(s) => s.hit_info(its),
            Self::Sphere(s) => s.hit_info(its),
        }
    }

    #[inline(always)] fn intersection_cost(&self) -> F {
        match self {
            Self::None => 0.,
            Self::Mesh(s) => s.intersection_cost(),
            Self::Sphere(s) => s.intersection_cost(),
        }
    }
}

impl From<Mesh> for ShapeType
{ #[inline(always)] fn from(s: Mesh) -> Self { Self::Mesh(s) } }

impl From<Sphere> for ShapeType
{ #[inline(always)] fn from(s: Sphere) -> Self { Self::Sphere(s) } }

impl Zero for ShapeType { const ZERO: Self = Self::None; }
