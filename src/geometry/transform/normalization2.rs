use std::ops::Mul;

use crate::types::*;


#[derive(Clone, Copy, Debug)]
pub struct Norm2 {
    _11: F, _13: F,
    _22: F, _23: F,
}

impl Norm2 {
    #[inline]
    pub fn translate(P2(x, y): F2) -> Norm2 {
        Norm2 {
            _11: 1.,   _13: x,
            _22: 1.,   _23: y,
        }
    }

    #[inline]
    pub fn scale(P2(x, y): F2) -> Norm2 {
        Norm2 {
            _11: x,   _13: 0.,
            _22: y,   _23: 0.,
        }
    }
}

impl Mul for Norm2 {
    type Output = Norm2;
    #[inline]
    fn mul(self, o: Norm2) -> Norm2 {
        Norm2 {
            _11: self._11 * o._11,   _13: self._11 * o._13 + self._13,
            _22: self._22 * o._22,   _23: self._22 * o._23 + self._23
        }
    }
}

impl Mul<F2> for Norm2 {
    type Output = F2;
    #[inline]
    fn mul(self, P2(x, y): F2) -> F2 {
        P2(self._11 * x + self._13,
           self._22 * y + self._23)
    }
}
