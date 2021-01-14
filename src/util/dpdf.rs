use std::iter;

#[allow(clippy::wildcard_imports)]
use graphite::*;

use super::vec::LowerBound;

#[derive(Debug, Default)]
pub struct DiscretePDF {
    cdf:   Box<[F]>,
    total: F,
}

impl DiscretePDF {
    pub fn new<'a, A, C>(iter: C, p: impl Fn(&'a A) -> F) -> Self
        where A: 'a,
              C: IntoIterator<Item=&'a A>,
    {
        let mut cdf =
            iter::once(0.).chain(iter.into_iter().map(p).scan(0., |c, a| {
                                                            *c += a;
                                                            Some(*c)
                                                        }))
                          .collect::<Vec<_>>().into_boxed_slice();
        let total = *cdf.last().unwrap();
        cdf.iter_mut().for_each(|p| *p /= total);
        Self { cdf, total }
    }

    #[inline] pub fn sample(&self, s: &mut F) -> (usize, F) {
        let idx = usize::of(Num::clamp(self.cdf.lower_bound(*s) - 1,
                                       0, I::of(self.cdf.len()) - 1));
        let ci = self.cdf[idx];
        let cj = self.cdf[idx + 1];
        let prob = cj - ci;
        *s = (*s - ci) / prob;
        (idx, prob)
    }

    #[inline] pub const fn total(&self) -> F { self.total }
}
