use std::ops::{Add, Sub, Mul, Div, Neg, Index};

use super::*;


#[derive(Clone, Copy, Debug)]
pub struct V(pub F3);

impl V {
    #[inline(always)]
    pub fn v(x: F, y: F, z: F) -> V {
        V(A3(x, y, z))
    }

    #[inline(always)]
    pub fn p(P(p): P) -> V {
        V(p)
    }

    #[inline(always)]
    pub fn x(&self) -> F {
        (self.0).0
    }

    #[inline(always)]
    pub fn y(&self) -> F {
        (self.0).1
    }

    #[inline(always)]
    pub fn z(&self) -> F {
        (self.0).2
    }

    #[inline(always)]
    pub fn dot(self, V(v): V) -> F {
        dot(self.0, v)
    }

    #[inline(always)]
    pub fn norm2(self) -> F {
        self.dot(self)
    }

    #[inline(always)]
    pub fn norm(self) -> F {
        self.norm2().sqrt()
    }

    #[inline(always)]
    pub fn unit(self) -> V {
        self / self.norm()
    }

    #[inline(always)]
    pub fn shiftl(self) -> V {
        let V(v) = self;
        V::v(v.1, v.2, v.0)
    }

    #[inline(always)]
    pub fn shiftr(self) -> V {
        let V(v) = self;
        V::v(v.2, v.0, v.1)
    }
}

impl Add for V {
    type Output = V;
    #[inline(always)]
    fn add(self, V(v): V) -> V {
        V(self.0 + v)
    }
}

impl Sub for V {
    type Output = V;
    #[inline(always)]
    fn sub(self, V(v): V) -> V {
        V(self.0 - v)
    }
}

impl Neg for V {
    type Output = V;
    #[inline(always)]
    fn neg(self) -> V {
        V(-self.0)
    }
}

impl Mul for V {
    type Output = V;
    #[inline(always)]
    fn mul(self, v: V) -> V {
        V(zip(self.shiftl().0, v.shiftr().0, Mul::mul) -
          zip(self.shiftr().0, v.shiftl().0, Mul::mul))
    }
}

impl Mul<V> for F {
    type Output = V;
    #[inline(always)]
    fn mul(self, V(v): V) -> V {
        V(self * v)
    }
}

impl Div<F> for V {
    type Output = V;
    #[inline(always)]
    fn div(self, f: F) -> V {
        V(self.0 / f)
    }
}

impl Mul<V> for T {
    type Output = V;
    #[inline(always)]
    fn mul(self, V(v): V) -> V {
        V(self.rot() * v)
    }
}

impl Div<V> for T {
    type Output = V;
    #[inline(always)]
    fn div(self, V(v): V) -> V {
        V(self.rot() / v)
    }
}

impl Index<Axis> for V {
    type Output = F;
    #[inline(always)]
    fn index(&self, axis: Axis) -> &F {
        &self.0[axis]
    }
}
