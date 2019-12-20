use std::iter;

use super::*;

pub struct DiscretePDF {
    cdf:   Vec<F>,
    total: F,
}

impl DiscretePDF {
    pub fn new<C: IntoIterator>(iter: C, p: impl Fn(C::Item) -> F) -> Self {
        let mut cdf =
            iter::once(0.).chain(iter.into_iter().map(p).scan(0., |c, a| {
                                                            *c += a;
                                                            Some(*c)
                                                        }))
                          .collect::<Vec<_>>();
        let total = *cdf.last().unwrap();
        cdf.iter_mut().for_each(|p| *p /= total);
        Self { cdf, total }
    }

    #[inline(always)]
    pub fn sample(&self, s: &mut F) -> usize {
        let idx = Num::clamp(self.cdf.lower_bound(*s) - 1,
                             0,
                             self.cdf.len() as I - 1)
                  as usize;
        let ci = self.cdf[idx];
        let cj = self.cdf[idx + 1];
        *s = (*s - ci) / (cj - ci);
        idx
    }

    #[inline(always)]
    pub fn total(&self) -> F { self.total }
}
