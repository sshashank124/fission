use std::ops::Try;

pub enum Either<A, B> {
    L(A),
    R(B),
}

impl<A, B> Try for Either<A, B> {
    type Ok = B;
    type Error = A;

    #[inline] fn into_result(self) -> Result<B, A> {
        match self {
            Self::L(l) => Err(l),
            Self::R(r) => Ok(r),
        }
    }

    #[inline] fn from_error(a: A) -> Self { Self::L(a) }
    #[inline] fn from_ok(b: B) -> Self { Self::R(b) }
}
