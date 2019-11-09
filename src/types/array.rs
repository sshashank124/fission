use std::ops::Index;

use super::*;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct A3<A>(pub A, pub A, pub A);

#[inline(always)]
pub fn rep<A: Copy>(a: A) -> A3<A> {
    A3(a, a, a)
}

#[inline(always)]
pub fn zip<A, B, Z, FN>(a: A3<A>, b: A3<B>, f: FN) -> A3<Z>
        where FN: Fn(A, B) -> Z {
    A3(f(a.0, b.0), f(a.1, b.1), f(a.2, b.2))
}

#[inline(always)]
pub fn zip3<A, B, C, Z, FN>(a: A3<A>, b: A3<B>, c: A3<C>, f: FN) -> A3<Z>
        where FN: Fn(A, B, C) -> Z {
    A3(f(a.0, b.0, c.0), f(a.1, b.1, c.1), f(a.2, b.2, c.2))
}

impl<A> A3<A> {
    #[inline(always)]
    pub fn map<Z,FN>(self, f: FN) -> A3<Z>
            where FN: Fn(A) -> Z {
        A3(f(self.0), f(self.1), f(self.2))
    }

    #[inline(always)]
    pub fn reduce<FN>(self, f: FN) -> A
            where FN: Fn(A, A) -> A {
        f(f(self.0, self.1), self.2)
    }

    #[inline(always)]
    pub fn apply<Z,FN>(self, f: FN) -> Z
            where FN: Fn(A, A, A) -> Z {
        f(self.0, self.1, self.2)
    }

    #[inline(always)]
    pub fn fold<Z,FN>(self, z: Z, f: FN) -> Z
            where FN: Fn(Z, A) -> Z {
        f(f(f(z, self.0), self.1), self.2)
    }
}

impl<A> A3<A3<A>> {
    #[inline(always)]
    pub fn unzip<Z, FN>(self, f: FN) -> A3<Z>
            where FN: Fn(A, A, A) -> Z {
        zip3(self.0, self.1, self.2, f)
    }

    #[inline(always)]
    pub fn t(self) -> A3<A3<A>> {
        self.unzip(A3)
    }
}

impl<A> Index<Axis> for A3<A> {
    type Output = A;
    #[inline(always)]
    fn index(&self, axis: Axis) -> &A {
        match axis {
            Axis::X => &self.0,
            Axis::Y => &self.1,
            Axis::Z => &self.2,
        }
    }
}
