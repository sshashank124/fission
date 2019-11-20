use std::ops::{Index, IndexMut};
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use std::ops::{BitAnd, BitOr};

use crate::*;
use super::*;


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct A3<A>(pub A, pub A, pub A);


// General Arrays

impl<A> A3<A> {
    #[inline(always)] pub fn rep(a: A) -> A3<A> where A: Copy
    { A3(a, a, a) }

    #[inline(always)] pub fn a2(a2: A2<A>, a: A) -> A3<A> where A: Copy
    { A3(a2[0], a2[1], a) }

    #[inline(always)] pub fn map<B, G>(self, f: G) -> A3<B> where G: Fn(A) -> B
    { A3(f(self.0), f(self.1), f(self.2)) }

    #[inline(always)]
    pub fn zip<B, C, G>(self, b: A3<B>, f: G) -> A3<C> where G: Fn(A, B) -> C
    { A3(f(self.0, b.0), f(self.1, b.1), f(self.2, b.2)) }

    #[inline(always)]
    pub fn zip3<B, C, D, G>(self, b: A3<B>, c: A3<C>, f: G) -> A3<D>
    where G: Fn(A, B, C) -> D
    { A3(f(self.0, b.0, c.0), f(self.1, b.1, c.1), f(self.2, b.2, c.2)) }

    #[inline(always)] pub fn reduce<G>(self, f: G) -> A where G: Fn(A, A) -> A
    { f(f(self.0, self.1), self.2) }

    #[inline(always)]
    pub fn fold<B, G>(self, acc: B, f: G) -> B where G: Fn(B, A) -> B
    { f(f(f(acc, self.0), self.1), self.2) }
}

impl<A> A3<A3<A>> {
    #[inline(always)]
    pub fn unzip<B, G>(self, f: G) -> A3<B> where G: Fn(A, A, A) -> B
    { self.0.zip3(self.1, self.2, f) }
}

macro_rules! index {
    ($type:ident, $v1:tt, $v2:tt, $v3:tt) => {
        impl<A> Index<$type> for A3<A> {
            type Output = A;
            #[inline(always)]
            fn index(&self, i: $type) -> &Self::Output {
                match i {
                    $v1 => &self.0, $v2 => &self.1, $v3 => &self.2,
                    #[allow(unreachable_patterns)] _ => unreachable!(),
                }
            }
        }

        impl<A> IndexMut<$type> for A3<A> {
            #[inline(always)]
            fn index_mut(&mut self, i: $type) -> &mut Self::Output {
                match i {
                    $v1 => &mut self.0, $v2 => &mut self.1, $v3 => &mut self.2,
                    #[allow(unreachable_patterns)] _ => unreachable!(),
                }
            }
        }
    }
}

index!(I, 0, 1, 2);
index!(usize, 0, 1, 2);
index!(Dim, X, Y, Z);



// Numeric Arrays

pub type F3 = A3<F>;

impl<A> Zero for A3<A> where A: Zero {
    const ZERO: Self = A3(A::ZERO, A::ZERO, A::ZERO);
}

impl<A> One for A3<A> where A: One {
    const ONE: Self = A3(A::ONE, A::ONE, A::ONE);
}

impl<A> A3<A> where A: Zero + One {
    pub const X: A3<A> = A3(A::ONE , A::ZERO, A::ZERO);
    pub const Y: A3<A> = A3(A::ZERO, A::ONE , A::ZERO);
    pub const Z: A3<A> = A3(A::ZERO, A::ZERO, A::ONE );
}

impl<A> A3<A> {
    #[inline(always)]
    pub fn dot<B, C>(self, b: A3<B>) -> C where A: Mul<B, Output=C>,
                                                C: Add<C, Output=C> {
        self.zip(b, Mul::mul).reduce(Add::add)
    }
}

macro_rules! cw_binary_assign_op {
    ($trait:ident, $op:ident) => {
        impl<A, B> $trait<A3<B>> for A3<A> where A: $trait<B>,
                                                 B: Copy {
            #[inline(always)]
            fn $op(&mut self, b: A3<B>) {
                $trait::$op(&mut self[X], b[X]);
                $trait::$op(&mut self[Y], b[Y]);
                $trait::$op(&mut self[Z], b[Z]);
            }
        }
    }
}

cw_unary_op!(A3, Neg, neg);
cw_unary_op!(A3, Inv, inv);

cw_binary_op!(A3, Add, add);
cw_binary_op!(A3, Sub, sub);
cw_binary_op!(A3, Mul, mul);
cw_binary_op!(A3, Div, div);
cw_binary_op!(A3, BitAnd, bitand);
cw_binary_op!(A3, BitOr, bitor);

cw_binary_assign_op!(AddAssign, add_assign);
cw_binary_assign_op!(SubAssign, sub_assign);
cw_binary_assign_op!(MulAssign, mul_assign);
cw_binary_assign_op!(DivAssign, div_assign);

scalar_binary_op!(A3, Add, add);
scalar_binary_op!(A3, Sub, sub);
scalar_binary_op!(A3, Mul, mul);
scalar_binary_op!(A3, Div, div);

scalar_binary_assign_op!(A3, AddAssign, add_assign);
scalar_binary_assign_op!(A3, SubAssign, sub_assign);
scalar_binary_assign_op!(A3, MulAssign, mul_assign);
scalar_binary_assign_op!(A3, DivAssign, div_assign);
