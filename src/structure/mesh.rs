use std::ops::BitOr;
use std::sync::Arc;

use super::*;


pub type Mesh = BVH<Triangle>;

#[derive(Debug)]
pub struct Triangle {
    pub f: Face,
    pub mesh_data: Arc<MeshData>,
}

#[derive(Debug)]
pub struct Face {
    a: I,
    b: I,
    c: I,
}

#[derive(Debug)]
pub struct MeshData {
    pub p:  Vec<P>,
    pub n:  Vec<N>,
    pub uv: Vec<F2>,
}

impl Face {
    #[inline(always)]
    pub fn new(a: I, b: I, c: I) -> Face {
        Face { a, b, c }
    }
}

impl MeshData {
    #[inline(always)]
    pub fn new() -> MeshData {
        MeshData {
            p:  Vec::new(),
            n:  Vec::new(),
            uv: Vec::new(),
        }
    }
}

impl Triangle {
    #[inline(always)]
    fn a(&self) -> P {
        self.mesh_data.p[self.f.a as usize]
    }

    #[inline(always)]
    fn b(&self) -> P {
        self.mesh_data.p[self.f.b as usize]
    }

    #[inline(always)]
    fn c(&self) -> P {
        self.mesh_data.p[self.f.c as usize]
    }

    #[inline(always)]
    fn abc(&self) -> A3<P> {
        A3(self.a(), self.b(), self.c())
    }

    #[inline(always)]
    fn ab(&self) -> V {
        self.b() - self.a()
    }

    #[inline(always)]
    fn ac(&self) -> V {
        self.c() - self.a()
    }

    #[inline(always)]
    fn n(&self) -> N {
        N(self.ab() * self.ac())
    }

    #[inline(always)]
    fn intersection_point(&self, r: R) -> Option<F> {
        let pv = r.d * self.ac();
        let det = self.ab().dot(pv);
        if det.abs() < F::EPSILON { return None; }

        let dinv = det.inv();
        let tv = r.o - self.a();
        let u = tv.dot(pv) * dinv;
        if u < 0. || u > 1. { return None; }

        let q = tv * self.ab();
        let v = r.d.dot(q) * dinv;
        if v < 0. || u + v > 1. { return None; }

        let t = self.ac().dot(q) * dinv;
        if r.tb.bounds(t) {
            Some(t)
        } else {
            None
        }
    }
}

impl Intersectable for Triangle {
    #[inline(always)]
    fn bbox(&self, t: T) -> BBox {
        self.abc().map(|vert| t * vert).fold(BBox::EMPTY, BitOr::bitor)
    }

    #[inline(always)]
    fn intersects(&self, r: R) -> bool {
        self.intersection_point(r).is_some()
    }

    #[inline(always)]
    fn intersect(&self, r: R) -> Option<Its> {
        self.intersection_point(r).map(|t| {
            Its::ideal(r.at(t), t, self.n())
        })
    }
}
