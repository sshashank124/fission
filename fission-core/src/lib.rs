#![feature(try_trait)]

#![warn(clippy::all,
        clippy::cargo,
        clippy::nursery,
        clippy::pedantic)]

#![allow(clippy::cast_possible_truncation,
         clippy::cast_possible_wrap,
         clippy::cast_precision_loss,
         clippy::cast_sign_loss,
         clippy::find_map,
         clippy::missing_errors_doc,
         clippy::multiple_crate_versions,
         clippy::must_use_candidate,
         clippy::non_ascii_literal,
         clippy::unreadable_literal)]

#![allow(clippy::cargo_common_metadata)]

mod aggregate;
mod bsdf;
mod camera;
mod image;
mod light;
pub mod renderer;
mod sampler;
mod scene;
mod shape;
mod texture;
mod tracer;
pub mod util;
