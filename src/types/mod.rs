mod array;
mod array_num;
mod axis;
mod color;
mod num;
mod point2;

pub use array::*;
pub use array_num::*;
pub use axis::*;
pub use color::Color;
pub use num::*;
pub use point2::P2;

pub type I = u32;
pub type F = f32;

pub type F3 = A3<F>;
pub type I2 = P2<I>;
pub type F2 = P2<F>;
