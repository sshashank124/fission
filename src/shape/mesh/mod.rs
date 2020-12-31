mod triangle;

use std::convert::TryFrom;

use objloader::{Face, MeshData};

use super::*;
use crate::util::{DiscretePDF, Either};

pub use triangle::*;

#[derive(Debug, Deserialize)]
#[serde(try_from="MeshConfig")]
pub struct Mesh {
    tris: BVH<Triangle>,
    dpdf: DiscretePDF,
}

impl Intersectable for Mesh {
    #[inline(always)] fn bbox(&self) -> BBox { self.tris.bbox() }

    #[inline(always)] fn intersects(&self, ray: R) -> bool
    { self.tris.intersects(ray) }

    #[inline(always)] fn intersect(&self, ray: R) -> Option<Its> {
        self.tris
            .fold(F3::from(ray.d).map(Num::is_pos),
                  (ray, None),
                  |(r, _), node| node.bbox.intersects(*r),
                  |acc, i, s| Either::R(intersect_update(acc, (i, s))))
            .1
    }

    #[inline(always)] fn hit_info<'a>(&'a self, i: Its<'a>) -> Its<'a> {
        self.tris.elements[i.shape.1 as usize].hit_info(i)
    }

    #[inline(always)] fn sample_surface(&self, mut s: F2) -> Its {
        let idx = self.dpdf.sample(&mut s[0]);
        self.tris.elements[idx].sample_surface(s)
    }

    #[inline(always)] fn surface_area(&self) -> F { self.dpdf.total() }

    fn intersection_cost(&self) -> F { self.tris.intersection_cost() }
}

type Acc<'a> = (R, Option<Its<'a>>);
#[inline(always)] pub fn intersect_update<'a>((ray, acc): Acc<'a>,
                            (i, s): (usize, &'a impl Intersectable))
                            -> Acc<'a> {
    s.intersect(ray)
     .map(|it| (ray.clipped(it.t), Some(it.for_idx(i))))
     .unwrap_or_else(|| (ray, acc))
}


#[derive(Debug, Deserialize)]
struct MeshConfig {
    obj: String,
    #[serde(default)]
    transforms: Vec<T>,
}

impl TryFrom<MeshConfig> for Mesh {
    type Error = Error;

    fn try_from(mc: MeshConfig) -> Result<Self> {
        let to_world = T::product(mc.transforms.into_iter());
        let (mesh_data, faces) = objloader::load_from_file(&mc.obj, to_world)?;
        let mesh_data = Arc::new(mesh_data);
        let triangles = faces.into_iter().map(|f| Triangle {
                                 f,
                                 mesh_data: mesh_data.clone()
                             }).collect();
        let tris = BVH::new(triangles);
        let dpdf = DiscretePDF::new(&tris.elements, Triangle::surface_area);
        Ok(Self { tris, dpdf })
    }
}
