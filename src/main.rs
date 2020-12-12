#![feature(try_trait)]
#![allow(clippy::suspicious_arithmetic_impl)]

mod aggregate;
mod bsdf;
mod camera;
mod config;
mod image;
mod integrator;
mod light;
mod sampler;
mod scene;
mod shape;
mod texture;
mod tracer;
mod util;

mod prelude {
    pub use graphite::*;
}

use std::env;
use std::path::Path;

use util::Progress;

fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        return Err("Usage: fission <scene_description.yaml>".into())
    }
    let config_file = &args[1];

    let integrator = {
        let _progress = Progress::new("Loading scene description", None);
        config::load_from_file(config_file)
               .map_err(|e| format!("Failed to load config: {}", e))?
    };

    let image = integrator.render();

    let save_path = Path::new(config_file).with_extension("exr");
    let save_name = save_path.to_str().unwrap();
    {
        let _progress = Progress::new("Saving rendered image", None);
        image.save_exr(save_name)
             .map_err(|e| format!("Saving image failed: {}", e))?;
    }

    Ok(())
}
