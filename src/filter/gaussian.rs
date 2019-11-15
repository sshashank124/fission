use super::*;


#[derive(Clone, Copy)]
pub struct Gaussian {
    radius: F,
    alpha: F,
    c: F,
}

impl Gaussian {
    #[inline(always)]
    pub fn new(radius: F, stdev: F) -> Self {
        let alpha = -1. / (2. * stdev.sq());
        let c = -F::exp(alpha * radius.sq());
        Self { radius, alpha, c }
    }
}

impl Filter for Gaussian {
    #[inline(always)] fn eval(&self, dist: F) -> F {
        F::max(0., F::exp(self.alpha * dist.sq()) + self.c)
    }

    #[inline(always)] fn radius(&self) -> F { self.radius }
}
