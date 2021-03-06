#[allow(clippy::wildcard_imports)]
use graphite::*;

pub trait LowerBound<A> { fn lower_bound(&self, value: A) -> I; }

impl<A> LowerBound<A> for &[A] where A: PartialOrd
{
    #[inline] fn lower_bound(&self, value: A) -> I {
        let mut l = self.len();
        if l == 0 { return 0 }
        let mut a = 0;
        while l > 1 {
            let hl = l / 2;
            let mid = a + hl;
            if value > self[mid] { a = mid; }
            l -= hl;
        }
        I::of(a) + I::of(value > self[a])
    }
}

impl<A> LowerBound<A> for Box<[A]> where A: PartialOrd
{ #[inline] fn lower_bound(&self, value: A) -> I { (&**self).lower_bound(value) } }
