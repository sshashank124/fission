use std::fmt::Display;


pub type Res<T> = Result<T, String>;

pub trait ErrorContext<C> {
    fn with_msg(self, msg: &str) -> C;
}

impl<T, E> ErrorContext<Res<T>> for Result<T, E> where E: Display {
    #[inline(always)]
    fn with_msg(self, msg: &str) -> Res<T> {
        self.map_err(|e| format!("{}: {}", msg, e))
    }
}
