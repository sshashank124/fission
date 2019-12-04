use scoped_threadpool::Pool;


pub trait Parallelize<I> { fn parallelize(self) -> ParIter<I>; }

impl<I> Parallelize<I> for I
{ #[inline(always)] fn parallelize(self) -> ParIter<I> { ParIter(self) } }


pub struct ParIter<I>(I);

impl<I> ParIter<I> where I: Iterator, I::Item: Send {
    #[inline(always)]
    pub fn for_each<FN>(self, f: FN) where FN: Fn(I::Item) + Sync
    { Pool::new(8).scoped(|s| self.0.for_each(|e| s.execute(|| f(e)))) }
}
