#![feature(test)]
#![allow(clippy::suspicious_arithmetic_impl)]

mod aggregate;
mod bsdf;
mod camera;
mod core;
mod geometry;
mod image;
mod integrator;
mod light;
mod loader;
mod parallel;
mod sampler;
mod scene;
mod shape;
mod texture;
mod tracer;

use std::env;
use std::path::Path;

use crate::core::*;
use loader::*;


fn main() -> Res<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2
    { return Err("Usage: fission <scene_description.yaml>".into()) }
    let config_file = &args[1];

    let load_progress = Progress::new("Loading scene description", None);
    let integrator = config::load_from_file(config_file)
                            .with_msg("Failed to load config")?;
    load_progress.finish();

    let image = integrator.render();

    let save_path = Path::new(config_file).with_extension("exr");
    let save_name = save_path.to_str().unwrap();
    let save_progress = Progress::new("Saving rendered image", None);
    image.save_exr(save_name).with_msg("Saving image failed")?;
    save_progress.finish();

    Ok(())
}
