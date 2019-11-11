use std::ops::{Add, Sub, Mul, Div, Neg};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

use super::*;


impl<N> Zero for A3<N> where N: Zero {
    const ZERO: Self = A3(N::ZERO, N::ZERO, N::ZERO);
}

impl<N> One for A3<N> where N: One {
    const ONE: Self = A3(N::ONE, N::ONE, N::ONE);
}

impl<N> A3<N> where N: Num {
    pub const X: A3<N> = A3(N::ONE,  N::ZERO, N::ZERO);
    pub const Y: A3<N> = A3(N::ZERO, N::ONE,  N::ZERO);
    pub const Z: A3<N> = A3(N::ZERO, N::ZERO, N::ONE );
}


#[inline(always)]
pub fn dot<A, B, C>(a: A3<A>, b: A3<B>) -> C
        where A: Mul<B, Output=C>,
              C: Add<C, Output=C> {
    zip(a, b, Mul::mul).reduce(Add::add)
}

#[macro_export]
macro_rules! op {
    ($trait:ident::$op:ident, *$type:ident) => {
        impl $trait for $type { type Output = $type; #[inline(always)]
            fn $op(self) -> $type { $type($trait::$op(*self)) } }
    };
    ($trait:ident::$op:ident, *$lhs:ident -> *$rhs:ident -> $out:ident) => {
        impl $trait<$rhs> for $lhs { type Output = $out; #[inline(always)]
            fn $op(self, b: $rhs) -> $out { $out($trait::$op(*self, *b)) }
        }
    };
    ($trait:ident::$op:ident, $lhs:ident -> *$rhs:ident -> $out:ident) => {
        impl $trait<$rhs> for $lhs { type Output = $out; #[inline(always)]
            fn $op(self, b: $rhs) -> $out { $out($trait::$op(self, *b)) }
        }
    };
    ($trait:ident::$op:ident, *$lhs:ident -> $rhs:ident -> $out:ident) => {
        impl $trait<$rhs> for $lhs { type Output = $out; #[inline(always)]
            fn $op(self, b: $rhs) -> $out { $out($trait::$op(*self, b)) }
        }
    }
}

macro_rules! cw_unary_op {
    ($trait:ident, $op:ident) => {
        impl<N> $trait for A3<N> where N: $trait<Output=N> {
            type Output = A3<N>;
            #[inline(always)]
            fn $op(self) -> A3<N> {
                self.map($trait::$op)
            }
        }
    }
}

macro_rules! cw_binary_op {
    ($trait:ident, $op:ident) => {
        impl<N> $trait for A3<N> where N: Num {
            type Output = A3<N>;
            #[inline(always)]
            fn $op(self, b: A3<N>) -> A3<N> {
                zip(self, b, $trait::$op)
            }
        }
    }
}

macro_rules! cw_binary_assign_op {
    ($trait:ident, $op:ident) => {
        impl<N> $trait for A3<N> where N: Num {
            #[inline(always)]
            fn $op(&mut self, b: A3<N>) {
                $trait::$op(&mut self.0, b.0);
                $trait::$op(&mut self.1, b.1);
                $trait::$op(&mut self.2, b.2);
            }
        }
    }
}

macro_rules! scalar_binary_op {
    ($trait:ident, $op:ident) => {
        impl<N> $trait<N> for A3<N> where N: Num {
            type Output = A3<N>;
            #[inline(always)]
            fn $op(self, b: N) -> A3<N> {
                $trait::$op(self, rep(b))
            }
        }
    }
}

macro_rules! scalar_binary_assign_op {
    ($trait:ident, $op:ident) => {
        impl<N> $trait<N> for A3<N> where N: Num {
            #[inline(always)]
            fn $op(&mut self, b: N) {
                $trait::$op(self, rep(b))
            }
        }
    }
}

cw_unary_op!(Neg, neg);
cw_unary_op!(Inv, inv);

cw_binary_op!(Add, add);
cw_binary_op!(Sub, sub);
cw_binary_op!(Mul, mul);
cw_binary_op!(Div, div);

cw_binary_assign_op!(AddAssign, add_assign);
cw_binary_assign_op!(SubAssign, sub_assign);
cw_binary_assign_op!(MulAssign, mul_assign);
cw_binary_assign_op!(DivAssign, div_assign);

scalar_binary_op!(Add, add);
scalar_binary_op!(Sub, sub);
scalar_binary_op!(Mul, mul);
scalar_binary_op!(Div, div);

scalar_binary_assign_op!(AddAssign, add_assign);
scalar_binary_assign_op!(SubAssign, sub_assign);
scalar_binary_assign_op!(MulAssign, mul_assign);
scalar_binary_assign_op!(DivAssign, div_assign);
