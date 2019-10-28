mod array;
mod color;
mod f3;
mod float;
mod point2;

pub use array::*;
pub use color::Color;
pub use f3::*;
pub use float::*;
pub use point2::P2;

pub type I = u32;
pub type F = f32;

pub type F3 = A3<F>;
pub type I2 = P2<I>;
pub type F2 = P2<F>;
