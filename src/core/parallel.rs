use scoped_threadpool::Pool;


pub trait Parallelize<I> { fn parallelize(self) -> ParIter<I>; }

impl<I> Parallelize<I> for I
{ #[inline(always)] fn parallelize(self) -> ParIter<I> { ParIter(self) } }


pub struct ParIter<I>(I);

impl<I> ParIter<I> where I: Iterator, I::Item: Send {
    #[inline(always)] pub fn for_each(self, f: impl Fn(I::Item) + Sync)
    { Pool::new(num_cpus::get() as u32)
           .scoped(|s| self.0.for_each(|e| s.execute(|| f(e)))) }
}
