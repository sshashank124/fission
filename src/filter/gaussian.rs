use super::*;


#[derive(Clone)]
pub struct Gaussian {
    radius: F,
    alpha: F,
    offset: F,
}

impl Gaussian {
    #[inline(always)]
    pub fn new(radius: F, stdev: F) -> Self {
        let alpha = -F::inv(2. * stdev.sq());
        let offset = -F::exp(alpha * radius.sq());
        Self { radius, alpha, offset }
    }
}

impl Filter for Gaussian {
    #[inline(always)] fn eval(&self, dist: F) -> F {
        F::clamp_pos(F::exp(self.alpha * dist.sq()) + self.offset)
    }

    #[inline(always)] fn radius(&self) -> F { self.radius }
}
