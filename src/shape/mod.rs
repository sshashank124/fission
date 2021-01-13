pub mod intersection;
mod mesh;
mod sphere;

use std::fmt;
use std::ops::BitAnd;

#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

use crate::bsdf::BSDF;
use crate::color::Color;
use crate::texture::Tex;

use intersection::Its;
use mesh::Mesh;
use sphere::Sphere;

pub trait Intersectable {
    fn bbox(&self) -> BBox;

    fn intersects(&self, ray: R) -> bool;
    fn intersect(&self, ray: R) -> Option<Its>;
    fn hit_info(&self, its: Its) -> Its;

    fn sample_surface(&self, s: F2) -> Its;
    fn surface_area(&self) -> F;
    #[inline] fn surface_pdf(&self) -> F { self.surface_area().inv() }

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

impl Shape {
    #[inline] pub const fn emits(&self) -> bool { self.emission.is_some() }
}

impl Intersectable for &'static Shape {
    #[inline] fn bbox(&self) -> BBox { self.shape.bbox() }

    #[inline] fn intersects(&self, ray: R) -> bool
    { self.shape.intersects(ray) }

    #[inline] fn intersect(&self, ray: R) -> Option<Its>
    { self.shape.intersect(ray).map(|its| its.for_shape(self)) }

    #[inline] fn hit_info(&self, its: Its) -> Its
    { self.shape.hit_info(its) }

    #[inline] fn sample_surface(&self, s: F2) -> Its
    { self.shape.sample_surface(s) }

    #[inline] fn surface_area(&self) -> F { self.shape.surface_area() }

    fn intersection_cost(&self) -> F { self.shape.intersection_cost() }
}

pub static PLACEHOLDER: Shape = Shape { shape: Type::ZERO, bsdf: BSDF::ZERO,
                                        emission: None };

#[derive(Deserialize)]
#[serde(tag="type", rename_all="snake_case")]
enum Type {
    None,
    Mesh(Mesh),
    Sphere(Sphere),
}

impl Intersectable for Type {
    #[inline] fn bbox(&self) -> BBox {
        match self {
            Self::None => unreachable!(),
            Self::Mesh(s) => s.bbox(),
            Self::Sphere(s) => s.bbox(),
        }
    }

    #[inline] fn intersects(&self, ray: R) -> bool {
        match self {
            Self::None => unreachable!(),
            Self::Mesh(s) => s.intersects(ray),
            Self::Sphere(s) => s.intersects(ray),
        }
    }

    #[inline] fn intersect(&self, ray: R) -> Option<Its> {
        match self {
            Self::None => unreachable!(),
            Self::Mesh(s) => s.intersect(ray),
            Self::Sphere(s) => s.intersect(ray),
        }
    }

    #[inline] fn hit_info(&self, its: Its) -> Its {
        match self {
            Self::None => unreachable!(),
            Self::Mesh(s) => s.hit_info(its),
            Self::Sphere(s) => s.hit_info(its),
        }
    }

    #[inline] fn sample_surface(&self, s: F2) -> Its {
        match self {
            Self::None => unreachable!(),
            Self::Mesh(sh) => sh.sample_surface(s),
            Self::Sphere(sh) => sh.sample_surface(s),
        }
    }

    #[inline] fn surface_area(&self) -> F {
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

impl Intersectable for BBox {
    #[inline] fn intersects(&self, ray: R) -> bool
    { !((*self - ray.o) / ray.d).0.fold(ray.range(), BitAnd::bitand).degen() }

    fn bbox(&self) -> BBox { unreachable!() }
    fn intersect(&self, _: R) -> Option<Its> { unreachable!() }
    fn hit_info(&self, _: Its) -> Its { unreachable!() }
    fn sample_surface(&self, _: F2) -> Its { unreachable!() }

    #[inline] fn surface_area(&self) -> F {
        let e = self.extents();
        2. * e[X].mul_add(e[Y], e[X].mul_add(e[Z], e[Y] + e[Z]))
    }

    fn intersection_cost(&self) -> F { 1. }
}
