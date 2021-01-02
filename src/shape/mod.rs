mod intersection;
mod mesh;
mod sphere;

use std::borrow::Borrow;
use std::fmt;
use std::ops::BitAnd;
pub use std::sync::Arc;

use crate::aggregate::*;
use crate::bsdf::*;
use crate::prelude::*;
use crate::texture::*;

pub use intersection::*;
pub use mesh::*;
pub use sphere::*;

pub trait Intersectable {
    fn bbox(&self) -> BBox;

    fn intersects(&self, ray: R) -> bool;
    fn intersect(&self, ray: R) -> Option<Its>;
    fn hit_info<'a>(&'a self, its: Its<'a>) -> Its<'a>;

    fn sample_surface(&self, s: F2) -> Its;
    fn surface_area(&self) -> F;
    #[inline(always)] fn surface_pdf(&self) -> F { self.surface_area().inv() }

    fn intersection_cost(&self) -> F;
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Shape {
    #[serde(flatten)]
        shape:    Type,
    pub bsdf:     BSDF,
    pub emission: Option<Tex<Color>>,
}

#[derive(Deserialize)]
#[serde(tag="type", rename_all="snake_case")]
enum Type {
    None,
    Mesh(Mesh),
    Sphere(Sphere),
}

impl Intersectable for Shape {
    #[inline(always)] fn bbox(&self) -> BBox { self.shape.bbox() }

    #[inline(always)] fn intersects(&self, ray: R) -> bool
    { self.shape.intersects(ray) }

    #[inline(always)] fn intersect(&self, ray: R) -> Option<Its>
    { self.shape.intersect(ray).map(|its| its.for_shape(self)) }

    #[inline(always)] fn hit_info<'a>(&'a self, its: Its<'a>) -> Its<'a>
    { self.shape.hit_info(its) }

    #[inline(always)] fn sample_surface(&self, s: F2) -> Its
    { self.shape.sample_surface(s) }

    #[inline(always)] fn surface_area(&self) -> F { self.shape.surface_area() }

    fn intersection_cost(&self) -> F { self.shape.intersection_cost() }
}

pub static SHAPE_PH: Shape = Shape { shape: Type::ZERO, bsdf: BSDF::ZERO,
                                     emission: None };

impl Intersectable for Type {
    #[inline(always)] fn bbox(&self) -> BBox {
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

    #[inline(always)] fn intersect(&self, ray: R) -> Option<Its> {
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

impl From<Mesh> for Type { fn from(s: Mesh) -> Self { Self::Mesh(s) } }

impl From<Sphere> for Type
{ fn from(s: Sphere) -> Self { Self::Sphere(s) } }

impl Zero for Type { const ZERO: Self = Self::None; }

impl Default for Type { fn default() -> Self { Self::None } }

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::None => "NoShape",
            Self::Mesh(_) => "Mesh",
            Self::Sphere(_) => "Sphere",
        })
    }
}

impl Intersectable for Arc<Shape> {
    #[inline(always)] fn bbox(&self) -> BBox { Shape::borrow(self).bbox() }

    #[inline(always)] fn intersects(&self, ray: R) -> bool
    { Shape::borrow(self).intersects(ray) }

    #[inline(always)] fn intersect(&self, ray: R) -> Option<Its>
    { Shape::borrow(self).intersect(ray).map(|its| its.for_shape(self)) }

    #[inline(always)] fn hit_info<'a>(&'a self, its: Its<'a>) -> Its<'a>
    { Shape::borrow(self).hit_info(its) }

    #[inline(always)] fn sample_surface(&self, s: F2) -> Its
    { Shape::borrow(self).sample_surface(s) }

    #[inline(always)] fn surface_area(&self) -> F
    { Shape::borrow(self).surface_area() }

    fn intersection_cost(&self) -> F { Shape::borrow(self).intersection_cost() }
}

impl Intersectable for BBox {
    #[inline(always)] fn intersects(&self, ray: R) -> bool
    { !((*self - ray.o) / ray.d).0.fold(ray.range(), BitAnd::bitand).degen() }

    fn bbox(&self) -> BBox { unreachable!() }
    fn intersect(&self, _: R) -> Option<Its> { unreachable!() }
    fn hit_info(&self, _: Its) -> Its { unreachable!() }
    fn sample_surface(&self, _: F2) -> Its { unreachable!() }

    #[inline(always)] fn surface_area(&self) -> F {
        let e = self.extents();
        2. * e[X].mul_add(e[Y], e[X].mul_add(e[Z], e[Y] + e[Z]))
    }

    fn intersection_cost(&self) -> F { 1. }
}
