#![feature(test)]
#![allow(clippy::suspicious_arithmetic_impl)]

mod aggregate;
mod bsdf;
mod camera;
mod geometry;
mod image;
mod integrator;
mod light;
mod loader;
mod sampler;
mod scene;
mod shape;
mod tracer;
mod types;
#[allow(dead_code)]
mod warp;

use std::env;
use std::path::Path;

use loader::*;


fn main() -> Res<()> {
    let args = env::args().collect::<Vec<_>>();
    
    if args.len() != 2 {
        return Err("Usage: fission <scene_description.yaml>".into());
    }

    let config_file = &args[1];
    let integrator = config::load_from_file(config_file)
                            .with_msg("Failed to load config")?;

    let image = integrator.render();

    print!("Saving rendered image ... ");
    let save_path = Path::new(config_file).with_extension("exr");
    let save_name = save_path.to_str()
                        .ok_or("Unable to create image save path")?;
    image.save_exr(save_name).with_msg("Saving image failed")?;
    println!("DONE");

    Ok(())
}
