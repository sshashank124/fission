use std::ops::{Index, IndexMut};
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use std::ops::{BitAnd, BitOr};

use crate::*;
use super::*;


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct A2<A>(pub A, pub A);


// General Arrays

impl<A> A2<A> {
    #[inline(always)] pub fn rep(a: A) -> A2<A> where A: Copy
    { A2(a, a) }

    #[inline(always)] pub fn map<B, G>(self, f: G) -> A2<B> where G: Fn(A) -> B
    { A2(f(self.0), f(self.1)) }

    #[inline(always)]
    pub fn zip<B, C, G>(self, b: A2<B>, f: G) -> A2<C> where G: Fn(A, B) -> C
    { A2(f(self.0, b.0), f(self.1, b.1)) }

    #[inline(always)]
    pub fn reduce<B, G>(self, f: G) -> B where G: Fn(A, A) -> B
    { f(self.0, self.1) }
}

macro_rules! index {
    ($type:ident, $v1:tt, $v2:tt) => {
        impl<A> Index<$type> for A2<A> {
            type Output = A;
            #[inline(always)] #[allow(clippy::match_bool)]
            fn index(&self, i: $type) -> &Self::Output {
                match i {
                    $v1 => &self.0, $v2 => &self.1,
                    #[allow(unreachable_patterns)] _ => unreachable!(),
                }
            }
        }

        impl<A> IndexMut<$type> for A2<A> {
            #[inline(always)] #[allow(clippy::match_bool)]
            fn index_mut(&mut self, i: $type) -> &mut Self::Output {
                match i {
                    $v1 => &mut self.0, $v2 => &mut self.1,
                    #[allow(unreachable_patterns)] _ => unreachable!(),
                }
            }
        }
    }
}

index!(I, 0, 1);
index!(usize, 0, 1);
index!(Dim, X, Y);
index!(bool, false, true);



// Numeric Arrays

pub type F2 = A2<F>;
pub type I2 = A2<I>;

impl<A> Zero for A2<A> where A: Zero {
    const ZERO: Self = A2(A::ZERO, A::ZERO);
}

impl<A> One for A2<A> where A: One {
    const ONE: Self = A2(A::ONE, A::ONE);
}

impl<A> Half for A2<A> where A: Half {
    const HALF: Self = A2(A::HALF, A::HALF);
}

impl<A> A2<A> {
    #[inline(always)]
    pub fn dot<B, C>(self, b: A2<B>) -> C where A: Mul<B, Output=C>,
                                                C: Add<C, Output=C>
    { self.zip(b, Mul::mul).reduce(Add::add) }

    #[inline(always)] pub fn sum(self) -> A where A: Add<A, Output=A>
    { self.reduce(Add::add) }

    #[inline(always)] pub fn mean(self) -> A where A: Add<A, Output=A>,
                                                   A: Mul<F, Output=A>
    { self.sum() * 0.5 }
}

macro_rules! cw_binary_assign_op {
    ($trait:ident, $op:ident) => {
        impl<A, B> $trait<A2<B>> for A2<A> where A: $trait<B>,
                                                 B: Copy {
            #[inline(always)]
            fn $op(&mut self, b: A2<B>) {
                $trait::$op(&mut self[X], b[X]);
                $trait::$op(&mut self[Y], b[Y]);
            }
        }
    }
}

cw_unary_op!(A2, Neg, neg);
cw_unary_op!(A2, Inv, inv);

cw_binary_op!(A2, Add, add);
cw_binary_op!(A2, Sub, sub);
cw_binary_op!(A2, Mul, mul);
cw_binary_op!(A2, Div, div);
cw_binary_op!(A2, BitAnd, bitand);
cw_binary_op!(A2, BitOr, bitor);

cw_binary_assign_op!(AddAssign, add_assign);
cw_binary_assign_op!(SubAssign, sub_assign);
cw_binary_assign_op!(MulAssign, mul_assign);
cw_binary_assign_op!(DivAssign, div_assign);

scalar_binary_op!(A2, Add, add);
scalar_binary_op!(A2, Sub, sub);
scalar_binary_op!(A2, Mul, mul);
scalar_binary_op!(A2, Div, div);

scalar_binary_assign_op!(A2, AddAssign, add_assign);
scalar_binary_assign_op!(A2, SubAssign, sub_assign);
scalar_binary_assign_op!(A2, MulAssign, mul_assign);
scalar_binary_assign_op!(A2, DivAssign, div_assign);

impl From<I2> for F2
{ #[inline(always)] fn from(a: I2) -> F2 { A2(a.0 as F, a.1 as F) } }
