#![feature(try_trait)]

mod aggregate;
mod bsdf;
mod camera;
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
    pub use serde::{Deserialize, Serialize};
}

use std::env;
use std::fs::File;
use std::path::Path;

use integrator::Integrator;
use util::Progress;

fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        return Err("Usage: fission <scene_description.yaml>".into())
    }

    let config_file = &args[1];
    let integrator: Integrator = {
        let _progress = Progress::new("Loading scene description", None);
        let f = File::open(config_file).map_err(|e| e.to_string())?;
        serde_yaml::from_reader(f).map_err(|e| e.to_string())?
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
