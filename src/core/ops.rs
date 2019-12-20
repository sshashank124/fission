#[macro_export]
macro_rules! op {
    ($trait:ident::$op:ident, *$type:ident) => {
        impl $trait for $type {
            type Output = $type;
            #[inline(always)]
            fn $op(self) -> $type { $type($trait::$op(*self)) }
        }
    };
    ($trait:ident::$op:ident, *$lhs:ident -> *$rhs:ident -> $out:ident) => {
        impl $trait<$rhs> for $lhs {
            type Output = $out;
            #[inline(always)]
            fn $op(self, b: $rhs) -> $out { $out($trait::$op(*self, *b)) }
        }
    };
    ($trait:ident::$op:ident, $lhs:ident -> *$rhs:ident -> $out:ident) => {
        impl $trait<$rhs> for $lhs {
            type Output = $out;
            #[inline(always)]
            fn $op(self, b: $rhs) -> $out { $out($trait::$op(self, *b)) }
        }
    };
    ($trait:ident::$op:ident, *$lhs:ident -> $rhs:ident -> $out:ident) => {
        impl $trait<$rhs> for $lhs {
            type Output = $out;
            #[inline(always)]
            fn $op(self, b: $rhs) -> $out { $out($trait::$op(*self, b)) }
        }
    };
    ($trait:ident::$op:ident, *mut $lhs:ident -> *$rhs:ident -> ()) => {
        impl $trait<$rhs> for $lhs {
            #[inline(always)]
            fn $op(&mut self, b: $rhs) { $trait::$op(&mut self.0, *b); }
        }
    };
    ($trait:ident::$op:ident, *mut $lhs:ident -> $rhs:ident -> ()) => {
        impl $trait<$rhs> for $lhs {
            #[inline(always)]
            fn $op(&mut self, b: $rhs) { $trait::$op(&mut self.0, b); }
        }
    };
}

#[macro_export]
macro_rules! cw_unary_op {
    ($array:ident, $trait:ident, $op:ident) => {
        impl<A> $trait for $array<A> where A: $trait<Output = A>
        {
            type Output = $array<A>;
            #[inline(always)]
            fn $op(self) -> Self::Output { self.map($trait::$op) }
        }
    };
}

#[macro_export]
macro_rules! cw_binary_op {
    ($array:ident, $trait:ident, $op:ident) => {
        impl<A, B, C> $trait<$array<B>> for $array<A>
            where A: $trait<B, Output = C>
        {
            type Output = $array<C>;
            #[inline(always)]
            fn $op(self, b: $array<B>) -> Self::Output {
                self.zip(b, $trait::$op)
            }
        }
    };
}

#[macro_export]
macro_rules! scalar_binary_op {
    ($array:ident, $trait:ident, $op:ident) => {
        impl<A, B, N> $trait<N> for $array<A>
            where A: $trait<N, Output = B>,
                  N: Num
        {
            type Output = $array<B>;
            #[inline(always)]
            fn $op(self, n: N) -> Self::Output {
                $trait::$op(self, $array::rep(n))
            }
        }
    };
}

#[macro_export]
macro_rules! scalar_binary_assign_op {
    ($array:ident, $trait:ident, $op:ident) => {
        impl<A, N> $trait<N> for $array<A>
            where N: Num,
                  A: $trait<N>
        {
            #[inline(always)]
            fn $op(&mut self, n: N) { $trait::$op(self, $array::rep(n)) }
        }
    };
}
