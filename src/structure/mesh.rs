use std::ops::BitOr;
use std::rc::Rc;

use super::*;
use aggregate::Aggregate;


pub type Mesh = Aggregate<Triangle, Vec<Triangle>>;

#[derive(Debug)]
pub struct Triangle {
    pub f: Face,
    pub mesh_data: Rc<MeshData>,
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
    #[inline]
    pub fn new(a: I, b: I, c: I) -> Face {
        Face { a, b, c }
    }
}

impl MeshData {
    #[inline]
    pub fn new() -> MeshData {
        MeshData {
            p:  Vec::new(),
            n:  Vec::new(),
            uv: Vec::new(),
        }
    }
}

impl Triangle {
    #[inline]
    fn a(&self) -> P {
        self.mesh_data.p[self.f.a as usize]
    }

    #[inline]
    fn b(&self) -> P {
        self.mesh_data.p[self.f.b as usize]
    }

    #[inline]
    fn c(&self) -> P {
        self.mesh_data.p[self.f.c as usize]
    }

    #[inline]
    fn abc(&self) -> A3<P> {
        A3(self.a(), self.b(), self.c())
    }

    #[inline]
    fn ab(&self) -> V {
        self.b() - self.a()
    }

    #[inline]
    fn ac(&self) -> V {
        self.c() - self.a()
    }

    #[inline]
    fn n(&self) -> N {
        N(self.ab() * self.ac())
    }

    #[inline]
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
    #[inline]
    fn bbox(&self, t: T) -> BBox {
        self.abc().map(|vert| t * vert).fold(BBox::EMPTY, BitOr::bitor)
    }

    #[inline]
    fn intersects(&self, r: R) -> bool {
        self.intersection_point(r).is_some()
    }

    #[inline]
    fn intersect(&self, r: R) -> Option<Its> {
        self.intersection_point(r).map(|t| {
            Its::ideal(r.at(t), t, self.n())
        })
    }
}
