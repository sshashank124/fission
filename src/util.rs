use std::fmt::Display;


pub type Res<T> = Result<T, String>;

pub trait ErrorContext<C> { fn with_msg(self, msg: &str) -> C; }

impl<T, E> ErrorContext<Res<T>> for Result<T, E> where E: Display {
    #[inline(always)] fn with_msg(self, msg: &str) -> Res<T>
    { self.map_err(|e| format!("{}: {}", msg, e)) }
}

#[inline(always)] pub fn tup<A, B>(a: A, b: B) -> (A, B) { (a, b) }

#[inline(always)]
pub fn tup_cmp_lt<A, B>((a1, b1): (A, B), (a2, b2): (A, B)) -> (A, B)
where
     A: PartialOrd
{
    if a1 < a2 { (a1, b1) } else { (a2, b2) }
}

#[inline(always)]
pub fn tup_cmp_gt<A, B>((a1, b1): (A, B), (a2, b2): (A, B)) -> (A, B)
where
     A: PartialOrd
{
    if a1 > a2 { (a1, b1) } else { (a2, b2) }
}
