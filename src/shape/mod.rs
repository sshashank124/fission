mod mesh;
mod sphere;

use std::borrow::Borrow;
use std::sync::Arc;

use crate::aggregate::*;
use crate::bsdf::*;
use crate::geometry::*;
use crate::medium::*;
use crate::texture::*;

pub use mesh::*;
pub use sphere::*;


pub trait Intersectable {
    fn bbox(&self) -> BBox;

    fn intersects(&self, ray: R) -> bool;
    fn intersect<'a>(&'a self, ray: R<'a>) -> Option<Its<'a>>;
    fn hit_info<'a>(&'a self, its: Its<'a>) -> Its<'a>;

    fn sample_surface(&self, s: F2) -> Its;
    fn surface_area(&self) -> F;
    #[inline(always)] fn surface_pdf(&self) -> F { self.surface_area().inv() }

    fn intersection_cost(&self) -> F;
}

pub struct Shape {
    shape: ShapeType,
    pub bsdf: BSDF,
    pub medium_interface: MediumInterface,
    pub emission: Option<Tex<Color>>,
}

pub enum ShapeType {
    None,
    Mesh(Mesh),
    Sphere(Sphere),
}

impl Shape {
    pub const fn new(shape: ShapeType, bsdf: BSDF,
                     medium_interface: MediumInterface,
                     emission: Option<Tex<Color>>) -> Self
    { Self { shape, bsdf, medium_interface, emission } }
}

impl Intersectable for Shape {
    fn bbox(&self) -> BBox { self.shape.bbox() }

    #[inline(always)] fn intersects(&self, ray: R) -> bool
    { self.shape.intersects(ray) }

    #[inline(always)] fn intersect<'a>(&'a self, ray: R<'a>) -> Option<Its<'a>>
    { self.shape.intersect(ray).map(|its| its.for_shape(self)) }

    #[inline(always)] fn hit_info<'a>(&'a self, its: Its<'a>) -> Its<'a>
    { self.shape.hit_info(its) }

    #[inline(always)] fn sample_surface(&self, s: F2) -> Its
    { self.shape.sample_surface(s) }

    #[inline(always)] fn surface_area(&self) -> F { self.shape.surface_area() }

    fn intersection_cost(&self) -> F { self.shape.intersection_cost() }
}

pub static SHAPE_DUMMY: Shape = Shape::new(ShapeType::ZERO, BSDF::ZERO,
                                           MediumInterface::ZERO, None);

impl Intersectable for ShapeType {
    fn bbox(&self) -> BBox {
        match self {
            Self::None => unreachable!(),
            Self::Mesh(s) => s.bbox(),
            Self::Sphere(s) => s.bbox(),
        }
    }

    #[inline(always)] fn intersects(&self, ray: R) -> bool {
        match self {
            Self::None => unreachable!(),
            Self::Mesh(s) => s.intersects(ray),
            Self::Sphere(s) => s.intersects(ray),
        }
    }

    #[inline(always)]
    fn intersect<'a>(&'a self, ray: R<'a>) -> Option<Its<'a>> {
        match self {
            Self::None => unreachable!(),
            Self::Mesh(s) => s.intersect(ray),
            Self::Sphere(s) => s.intersect(ray),
        }
    }

    #[inline(always)] fn hit_info<'a>(&'a self, its: Its<'a>) -> Its<'a> {
        match self {
            Self::None => unreachable!(),
            Self::Mesh(s) => s.hit_info(its),
            Self::Sphere(s) => s.hit_info(its),
        }
    }

    #[inline(always)] fn sample_surface(&self, s: F2) -> Its {
        match self {
            Self::None => unreachable!(),
            Self::Mesh(sh) => sh.sample_surface(s),
            Self::Sphere(sh) => sh.sample_surface(s),
        }
    }

    #[inline(always)] fn surface_area(&self) -> F {
        match self {
            Self::None => unreachable!(),
            Self::Mesh(s) => s.surface_area(),
            Self::Sphere(s) => s.surface_area(),
        }
    }

    fn intersection_cost(&self) -> F {
        match self {
            Self::None => unreachable!(),
            Self::Mesh(s) => s.intersection_cost(),
            Self::Sphere(s) => s.intersection_cost(),
        }
    }
}

impl From<Mesh> for ShapeType { fn from(s: Mesh) -> Self { Self::Mesh(s) } }

impl From<Sphere> for ShapeType
{ fn from(s: Sphere) -> Self { Self::Sphere(s) } }

impl Zero for ShapeType { const ZERO: Self = Self::None; }


impl Intersectable for Arc<Shape> {
    fn bbox(&self) -> BBox { Shape::borrow(self).bbox() }

    #[inline(always)] fn intersects(&self, ray: R) -> bool
    { Shape::borrow(self).intersects(ray) }

    #[inline(always)] fn intersect<'a>(&'a self, ray: R<'a>) -> Option<Its<'a>>
    { Shape::borrow(self).intersect(ray).map(|its| its.for_shape(self)) }

    #[inline(always)] fn hit_info<'a>(&'a self, its: Its<'a>) -> Its<'a>
    { Shape::borrow(self).hit_info(its) }

    #[inline(always)] fn sample_surface(&self, s: F2) -> Its
    { Shape::borrow(self).sample_surface(s) }

    #[inline(always)] fn surface_area(&self) -> F
    { Shape::borrow(self).surface_area() }

    fn intersection_cost(&self) -> F
    { Shape::borrow(self).intersection_cost() }
}
