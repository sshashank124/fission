use std::ops::{Index, IndexMut};

use super::*;


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct A3<A>(pub A, pub A, pub A);


#[inline(always)] pub fn rep<A: Copy>(a: A) -> A3<A> { A3(a, a, a) }

#[inline(always)]
pub fn zip<A, B, C, FN>(a: A3<A>, b: A3<B>, f: FN) -> A3<C>
where FN: Fn(A, B) -> C { A3(f(a.0, b.0), f(a.1, b.1), f(a.2, b.2)) }

#[inline(always)]
pub fn zip3<A, B, C, D, FN>(a: A3<A>, b: A3<B>, c: A3<C>, f: FN) -> A3<D>
where FN: Fn(A, B, C) -> D {
    A3(f(a.0, b.0, c.0), f(a.1, b.1, c.1), f(a.2, b.2, c.2))
}

#[inline(always)]
pub fn map<A, B, FN>(a: A3<A>, f: FN) -> A3<B>
where FN: Fn(A) -> B { A3(f(a.0), f(a.1), f(a.2)) }

#[inline(always)]
pub fn reduce<A, FN>(a: A3<A>, f: FN) -> A
where FN: Fn(A, A) -> A { f(f(a.0, a.1), a.2) }

#[inline(always)]
pub fn fold<A, B, FN>(a: A3<A>, acc: B, f: FN) -> B
where FN: Fn(B, A) -> B { f(f(f(acc, a.0), a.1), a.2) }

#[inline(always)]
pub fn unzip<A, B, FN>(a: A3<A3<A>>, f: FN) -> A3<B>
where FN: Fn(A, A, A) -> B { zip3(a.0, a.1, a.2, f) }


impl<A> A3<A> {
    #[inline(always)]
    pub fn zip<B, C, FN>(self, b: A3<B>, f: FN) -> A3<C>
    where FN: Fn(A, B) -> C { zip(self, b, f) }

    #[inline(always)]
    pub fn zip3<B, C, D, FN>(self, b: A3<B>, c: A3<C>, f: FN) -> A3<D>
    where FN: Fn(A, B, C) -> D { zip3(self, b, c, f) }

    #[inline(always)]
    pub fn map<B, FN>(self, f: FN) -> A3<B>
    where FN: Fn(A) -> B { map(self, f) }

    #[inline(always)]
    pub fn reduce<FN>(self, f: FN) -> A
    where FN: Fn(A, A) -> A { reduce(self, f) }

    #[inline(always)]
    pub fn fold<B, FN>(self, acc: B, f: FN) -> B
    where FN: Fn(B, A) -> B { fold(self, acc, f) }
}

impl<A> A3<A3<A>> {
    #[inline(always)]
    pub fn unzip<B, FN>(self, f: FN) -> A3<B>
    where FN: Fn(A, A, A) -> B { unzip(self, f) }
}


macro_rules! index {
    ($type:ident, $v1:tt, $v2:tt, $v3:tt) => {
        impl<A> Index<$type> for A3<A> {
            type Output = A;
            #[inline(always)]
            fn index(&self, i: $type) -> &Self::Output {
                match i {
                    $v1 => &self.0,
                    $v2 => &self.1,
                    $v3 => &self.2,
                    _ => unreachable!(),
                }
            }
        }

        impl<A> IndexMut<$type> for A3<A> {
            #[inline(always)]
            fn index_mut(&mut self, i: $type) -> &mut Self::Output {
                match i {
                    $v1 => &mut self.0,
                    $v2 => &mut self.1,
                    $v3 => &mut self.2,
                    _ => unreachable!(),
                }
            }
        }
    }
}

index!(I, 0, 1, 2);
index!(usize, 0, 1, 2);
index!(Dim, X, Y, Z);
