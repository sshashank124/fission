use std::ops::Deref;

use super::*;

#[derive(Clone, Copy, Debug)]
pub struct RotScale3(Option<A3<F3>>);

impl One for RotScale3 {
    const ONE: Self = Self(None);
}

impl RotScale3 {
    #[inline(always)]
    pub const fn new(r1: F3, r2: F3, r3: F3) -> Self {
        Self(Some(A3(r1, r2, r3)))
    }

    #[inline(always)]
    pub fn from_cols(c1: F3, c2: F3, c3: F3) -> Self {
        Self::new(c1, c2, c3).t()
    }

    pub fn scale(s: F3) -> Self { Self::new(F3::X * s, F3::Y * s, F3::Z * s) }

    pub fn rotate(axis: F3, theta: F) -> Self {
        let V(A3(x, y, z)) = V(axis).unit();
        let ct = theta.cosd();
        let cc = 1. - ct;
        let st = theta.sind();
        Self::new(A3(ct + x.sq() * cc,
                     x * y * cc - z * st,
                     x * z * cc + y * st),
                  A3(y * x * cc + z * st,
                     ct + y.sq() * cc,
                     y * z * cc - x * st),
                  A3(z * x * cc - y * st,
                     z * y * cc + x * st,
                     ct + z.sq() * cc))
    }

    #[inline(always)]
    pub fn from_frame(v: V) -> Self {
        let v2 = V(if F::abs(v[X]) > F::abs(v[Y]) {
            A3(-v[Z], 0., v[X]) / F::sqrt(v[X].sq() + v[Z].sq())
        } else {
            A3(0., v[Z], -v[Y]) / F::sqrt(v[Y].sq() + v[Z].sq())
        });
        let v3 = v.cross(v2);
        Self::from_cols(*v2, *v3, *v)
    }

    pub fn look_at(dir: V, up: V) -> Self {
        let dir = dir.unit();
        let right = (up.unit().cross(dir)).unit();
        let up = (dir.cross(right)).unit();
        Self::from_cols(*right, *up, *dir)
    }

    #[inline(always)]
    pub fn t(&self) -> Self { Self(self.map(|m| m.unzip(A3))) }
}

impl Mul for RotScale3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, o: Self) -> Self {
        self.t().map(|m| Self(Some(o * m)).t()).unwrap_or_else(|| o)
    }
}

impl<B> Mul<A3<B>> for RotScale3
    where B: Copy + Mul<F, Output = B> + Add<Output = B>
{
    type Output = A3<B>;
    #[inline(always)]
    fn mul(self, o: A3<B>) -> A3<B> {
        self.map(|m| A3::rep(o).zip(m, A3::dot)).unwrap_or_else(|| o)
    }
}

impl Deref for RotScale3 {
    type Target = Option<A3<F3>>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.0 }
}
