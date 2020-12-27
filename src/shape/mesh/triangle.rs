use std::fmt;
use std::ops::BitOr;
use std::sync::Arc;

use super::*;

pub struct Triangle {
    pub f:         Face,
    pub mesh_data: Arc<MeshData>,
}

impl Triangle {
    #[inline(always)] fn a(&self) -> P { self.mesh_data.p[self.f[0] as usize] }
    #[inline(always)] fn b(&self) -> P { self.mesh_data.p[self.f[1] as usize] }
    #[inline(always)] fn c(&self) -> P { self.mesh_data.p[self.f[2] as usize] }

    #[inline(always)] fn an(&self) -> N { self.mesh_data.n[self.f[0] as usize] }
    #[inline(always)] fn bn(&self) -> N { self.mesh_data.n[self.f[1] as usize] }
    #[inline(always)] fn cn(&self) -> N { self.mesh_data.n[self.f[2] as usize] }

    #[inline(always)] fn at(&self) -> F2
    { self.mesh_data.uv[self.f[0] as usize] }
    #[inline(always)] fn bt(&self) -> F2
    { self.mesh_data.uv[self.f[1] as usize] }
    #[inline(always)] fn ct(&self) -> F2
    { self.mesh_data.uv[self.f[2] as usize] }

    #[inline(always)] fn abc(&self) -> A3<P>
    { A3(self.a(), self.b(), self.c()) }
    #[inline(always)] fn abcn(&self) -> A3<N>
    { A3(self.an(), self.bn(), self.cn()) }
    #[inline(always)] fn abct(&self) -> A3<F2>
    { A3(self.at(), self.bt(), self.ct()) }

    #[inline(always)] fn ab(&self) -> V { self.b() - self.a() }
    #[inline(always)] fn ac(&self) -> V { self.c() - self.a() }

    #[inline(always)] fn n(&self) -> V { self.ab() * self.ac() }

    #[inline(always)] fn bary(uv: F2) -> F3
    { A3(F::ONE - uv[0] - uv[1], uv[0], uv[1]) }

    #[inline(always)] fn eval(&self, uv: F2) -> (P, N, F2) {
        let bary = Self::bary(uv);
        let p = A3::inner_product(self.abc(), bary);
        let n = if self.mesh_data.n.is_empty() { N::from(self.n()) }
                else { A3::inner_product(self.abcn(), bary) };
        let uv = if self.mesh_data.uv.is_empty() { F2::ZERO }
                 else { A3::inner_product(self.abct(), bary) };
        (p, n, uv)
    }

    #[inline(always)] fn intersection_point(&self, ray: R) -> Option<(F, F2)> {
        let pv = ray.d * self.ac();
        let det = F3::dot(self.ab(), pv);
        if F::approx_zero(det) { return None }

        let dinv = det.inv();
        let tv = ray.o - self.a();
        let u = F3::dot(tv, pv) * dinv;
        if !B::b(0., 1.).bounds(u) { return None }

        let q = tv * self.ab();
        let v = F3::dot(ray.d, q) * dinv;
        if v < 0. || u + v > 1. { return None }

        let t = F3::dot(self.ac(), q) * dinv;
        if ray.range().bounds(t) { Some((t, A2(u, v))) } else { None }
    }
}

impl Intersectable for Triangle {
    #[inline(always)] fn bbox(&self) -> BBox
    { self.abc().fold(BBox::ZERO, BitOr::bitor) }

    #[inline(always)] fn intersects(&self, ray: R) -> bool
    { self.intersection_point(ray).is_some() }

    #[inline(always)] fn intersect(&self, ray: R) -> Option<Its> {
        self.intersection_point(ray)
            .map(|(t, uv)| Its::new(P::ZERO, N::ZERO, uv, t))
    }

    #[inline(always)] fn hit_info<'a>(&'a self, mut its: Its<'a>) -> Its<'a> {
        let (p, n, uv) = self.eval(its.uv);
        its.p = p;
        its.n = n;
        its.uv = uv;
        its
    }

    #[inline(always)] fn sample_surface(&self, s: F2) -> Its {
        let (p, n, uv) = self.eval(UniformTriangle::warp(s));
        Its::new(p, n, uv, 0.)
    }

    #[inline(always)] fn surface_area(&self) -> F { 0.5 * self.n().norm() }

    fn intersection_cost(&self) -> F { 2. }
}

impl fmt::Debug for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { writeln!(f, "T") }
}
