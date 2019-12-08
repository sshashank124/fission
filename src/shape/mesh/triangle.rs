use std::ops::{BitOr, Deref};
use std::sync::Arc;

use super::*;


pub struct Triangle {
    pub f: Face,
    pub mesh_data: Arc<MeshData>,
}

pub struct Face(pub I, pub I, pub I);

pub struct MeshData {
    pub p:  Vec<P>,
    pub n:  Vec<N>,
    pub uv: Vec<F2>,
}

impl MeshData {
    pub fn new() -> MeshData
    { MeshData { p: Vec::new(), n: Vec::new(), uv: Vec::new() } }
}

impl Triangle {
    #[inline(always)] fn a(&self) -> P { self.p[self.f.0 as usize] }
    #[inline(always)] fn b(&self) -> P { self.p[self.f.1 as usize] }
    #[inline(always)] fn c(&self) -> P { self.p[self.f.2 as usize] }

    #[inline(always)] fn an(&self) -> N { self.n[self.f.0 as usize] }
    #[inline(always)] fn bn(&self) -> N { self.n[self.f.1 as usize] }
    #[inline(always)] fn cn(&self) -> N { self.n[self.f.2 as usize] }

    #[inline(always)] fn at(&self) -> F2 { self.uv[self.f.0 as usize] }
    #[inline(always)] fn bt(&self) -> F2 { self.uv[self.f.1 as usize] }
    #[inline(always)] fn ct(&self) -> F2 { self.uv[self.f.2 as usize] }

    #[inline(always)]
    fn abc(&self) -> A3<P> { A3(self.a(), self.b(), self.c()) }
    #[inline(always)]
    fn abcn(&self) -> A3<N> { A3(self.an(), self.bn(), self.cn()) }
    #[inline(always)]
    fn abct(&self) -> A3<F2> { A3(self.at(), self.bt(), self.ct()) }

    #[inline(always)] fn ab(&self) -> V { self.b() - self.a() }
    #[inline(always)] fn ac(&self) -> V { self.c() - self.a() }

    #[inline(always)] fn n(&self) -> V { self.ab().cross(self.ac()) }

    #[inline(always)] fn bary(uv: F2) -> F3
    { A3(F::ONE - uv[0] - uv[1], uv[0], uv[1]) }

    #[inline(always)] fn eval(&self, uv: F2) -> (P, N, F2) {
        let bary = Self::bary(uv);
        let p = self.abc().dot(bary);
        let n = if self.n.is_empty() { N::v(self.n()) }
                else { self.abcn().dot(bary) };
        let uv = if self.uv.is_empty() { F2::ZERO }
                 else { self.abct().dot(bary) };
        (p, n, uv)
    }

    #[inline(always)]
    fn intersection_point(&self, ray: R) -> Option<(F, F2)> {
        let pv = ray.d.cross(self.ac());
        let det = self.ab().dot(pv);
        if F::approx_zero(det) { return None; }

        let dinv = det.inv();
        let tv = ray.o - self.a();
        let u = tv.dot(pv) * dinv;
        if u < 0. || u > 1. { return None; }

        let q = tv.cross(self.ab());
        let v = ray.d.dot(q) * dinv;
        if v < 0. || u + v > 1. { return None; }

        let t = self.ac().dot(q) * dinv;
        if ray.range().bounds(t) { Some((t, A2(u, v))) }
        else { None }
    }
}

impl Intersectable for Triangle {
    fn bbox(&self) -> BBox { self.abc().fold(BBox::ZERO, BitOr::bitor) }

    #[inline(always)] fn intersects(&self, ray: R) -> bool
    { self.intersection_point(ray).is_some() }

    #[inline(always)] fn intersect(&self, ray: R) -> Option<Its>
    { self.intersection_point(ray)
          .map(|(t, uv)| Its::new(P::ZERO, N::ZERO, uv, t)) }

    #[inline(always)]
    fn hit_info<'a>(&'a self, mut its: Its<'a>) -> Its<'a> {
        let (p, n, uv) = self.eval(its.uv);
        its.p = p; its.n = n; its.uv = uv;
        its
    }

    #[inline(always)] fn sample_surface(&self, s: F2) -> Its {
        let (p, n, uv) = self.eval(UniformTriangle::warp(s, ()));
        Its::new(p, n, uv, 0.)
    }

    #[inline(always)] fn surface_area(&self) -> F { 0.5 * self.n().norm() }

    fn intersection_cost(&self) -> F { 2. }
}

impl Deref for Triangle { type Target = Arc<MeshData>;
    #[inline(always)] fn deref(&self) -> &Self::Target { &self.mesh_data }
}
