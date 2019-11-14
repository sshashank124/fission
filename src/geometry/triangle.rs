use std::ops::{BitOr, Deref};
use std::sync::Arc;

use super::*;


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
    #[inline(always)] fn a(&self) -> P { self.p[self.f.a as usize] }
    #[inline(always)] fn b(&self) -> P { self.p[self.f.b as usize] }
    #[inline(always)] fn c(&self) -> P { self.p[self.f.c as usize] }

    #[inline(always)] fn an(&self) -> N { self.n[self.f.a as usize] }
    #[inline(always)] fn bn(&self) -> N { self.n[self.f.b as usize] }
    #[inline(always)] fn cn(&self) -> N { self.n[self.f.c as usize] }

    #[inline(always)]
    fn abc(&self) -> A3<P> { A3(self.a(), self.b(), self.c()) }
    #[inline(always)]
    fn abcn(&self) -> A3<N> { A3(self.an(), self.bn(), self.cn()) }

    #[inline(always)] fn ab(&self) -> V { self.b() - self.a() }
    #[inline(always)] fn ac(&self) -> V { self.c() - self.a() }

    #[inline(always)] fn n(&self) -> N { N(self.ab().cross(self.ac())) }

    #[inline(always)]
    fn intersection_point(&self, ray: R) -> Option<(F, F2)> {
        let pv = ray.d.cross(self.ac());
        let det = self.ab().dot(pv);
        if det.abs() < F::EPSILON { return None; }

        let dinv = det.inv();
        let tv = ray.o - self.a();
        let u = tv.dot(pv) * dinv;
        if u < 0. || u > 1. { return None; }

        let q = tv.cross(self.ab());
        let v = ray.d.dot(q) * dinv;
        if v < 0. || u + v > 1. { return None; }

        let t = self.ac().dot(q) * dinv;
        if ray.tb.bounds(t) {
            Some((t, P2(u, v)))
        } else {
            None
        }
    }
}

impl Intersectable for Triangle {
    #[inline(always)]
    fn bbox(&self, t: T) -> BBox {
        self.abc().map(|vert| t * vert).fold(BBox::ZERO, BitOr::bitor)
    }

    #[inline(always)]
    fn intersects(&self, ray: R) -> bool {
        self.intersection_point(ray).is_some()
    }

    #[inline(always)]
    fn intersect(&self, ray: R) -> Option<Its> {
        self.intersection_point(ray).map(|(t, P2(u, v))| {
            let bary = A3(1. - u - v, u, v);
            let p = dot(self.abc(), bary);
            let n = if self.n.is_empty() { self.n() }
                    else { dot(self.abcn(), bary) };
            Its::new(p, t, n)
        })
    }
}

impl Deref for Triangle {
    type Target = Arc<MeshData>;
    #[inline(always)] fn deref(&self) -> &Self::Target { &self.mesh_data }
}
