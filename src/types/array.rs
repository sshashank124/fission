use std::ops::{Index, IndexMut};

use super::*;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct A3<A>(pub A, pub A, pub A);

#[inline(always)]
pub fn rep<A: Copy>(a: A) -> A3<A> { A3(a, a, a) }

#[inline(always)]
pub fn zip<A, B, C, FN>(a: A3<A>, b: A3<B>, f: FN) -> A3<C>
        where FN: Fn(A, B) -> C {
    A3(f(a.0, b.0), f(a.1, b.1), f(a.2, b.2))
}

#[inline(always)]
pub fn zip3<A, B, C, D, FN>(a: A3<A>, b: A3<B>, c: A3<C>, f: FN) -> A3<D>
        where FN: Fn(A, B, C) -> D {
    A3(f(a.0, b.0, c.0), f(a.1, b.1, c.1), f(a.2, b.2, c.2))
}

impl<A> A3<A> {
    #[inline(always)]
    pub fn map<B, FN>(self, f: FN) -> A3<B> where FN: Fn(A) -> B {
        A3(f(self.0), f(self.1), f(self.2))
    }

    #[inline(always)]
    pub fn reduce<FN>(self, f: FN) -> A where FN: Fn(A, A) -> A {
        f(f(self.0, self.1), self.2)
    }

    #[inline(always)]
    pub fn fold<B, FN>(self, b: B, f: FN) -> B where FN: Fn(B, A) -> B {
        f(f(f(b, self.0), self.1), self.2)
    }
}

impl<A> A3<A3<A>> {
    #[inline(always)]
    pub fn unzip<B, FN>(self, f: FN) -> A3<B> where FN: Fn(A, A, A) -> B {
        zip3(self.0, self.1, self.2, f)
    }

    #[inline(always)]
    pub fn t(self) -> A3<A3<A>> { self.unzip(A3)
    }
}

impl<A> Index<Dim> for A3<A> {
    type Output = A;
    #[inline(always)]
    fn index(&self, dim: Dim) -> &A {
        match dim {
            X => &self.0,
            Y => &self.1,
            Z => &self.2,
        }
    }
}

impl<A> IndexMut<Dim> for A3<A> {
    #[inline(always)]
    fn index_mut(&mut self, dim: Dim) -> &mut A {
        match dim {
            X => &mut self.0,
            Y => &mut self.1,
            Z => &mut self.2,
        }
    }
}
