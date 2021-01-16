use std::iter::Sum;
use std::ops::{AddAssign, Index, IndexMut};

#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::{Deserialize, Serialize};

use super::rect::Rect;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Bitmap<A> {
        data: Box<[A]>,
    pub rect: Rect,
}

impl<A> Bitmap<A> where A: Clone + Zero + AddAssign {
    #[inline] pub fn new(dims: I2) -> Self
    { Self::from_iter(Rect::new(I2::ZERO, dims), std::iter::empty()) }

    #[inline] pub fn from_iter<It>(rect: Rect, it: It) -> Self
        where It: IntoIterator<Item=(F2, A)>
    {
        let data = vec![A::ZERO; rect.area().conv()].into_boxed_slice();
        let mut block = Self { data, rect };
        it.into_iter().for_each(|(pos, item)| block[pos] += item);
        block
    }
}

impl<A> Bitmap<A> {
    #[inline] pub fn pixels(&self) -> impl Iterator<Item=&A> { self.data.iter() }
}

impl<A, B> AddAssign<Bitmap<B>> for Bitmap<A> where A: AddAssign<B>, B: Copy {
    #[inline] fn add_assign(&mut self, block: Bitmap<B>)
    { block.rect.positions().for_each(|pos| self[pos] += block[pos]); }
}

impl<A> Index<I2> for Bitmap<A> {
    type Output = A;
    #[inline] fn index(&self, pos: I2) -> &A
    { &self.data[usize::of(self.rect.flatten_abs_pos(pos))] }
}

impl<A> IndexMut<I2> for Bitmap<A> {
    #[inline] fn index_mut(&mut self, pos: I2) -> &mut A {
        let idx = usize::of(self.rect.flatten_abs_pos(pos));
        &mut self.data[idx]
    }
}

impl<A> Index<F2> for Bitmap<A> {
    type Output = A;
    #[inline] fn index(&self, pos: F2) -> &A { &self[pos.map(F::floori)] }
}

impl<A> IndexMut<F2> for Bitmap<A> {
    #[inline] fn index_mut(&mut self, pos: F2) -> &mut A { &mut self[pos.map(F::floori)] }
}

impl<A> Sum for Bitmap<A> where Bitmap<A>: Default + AddAssign {
    fn sum<It>(it: It) -> Self where It: Iterator<Item=Bitmap<A>> {
        let mut it = it.filter(|b| b.rect.dims != I2::ZERO);
        match it.next() {
            None => Self::default(),
            Some(mut acc) => { it.for_each(|b| acc += b); acc }
        }
    }
}
