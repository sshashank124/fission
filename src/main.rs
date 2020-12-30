#![feature(try_trait)]

mod aggregate;
mod bsdf;
mod camera;
mod image;
mod light;
mod renderer;
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
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use renderer::{Integrator, Renderer};
use util::Progress;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let running = Arc::new(AtomicBool::new(true));

    let r = running.clone();
    ctrlc::set_handler(move || { r.store(false, Ordering::SeqCst); })?;

    // Parse Args
    let mut args = env::args();
    if let None = args.next() { return Err("what".into()) }

    let config_file = match args.next() {
        Some(arg) => arg,
        None => return Err("Usage: fission <scene_description.yaml> \
                            [render_progress.state]".into())
    };
    let config_path = Path::new(&config_file);

    let state_file = args.next();

    // Load Integrator
    let integrator: Integrator = {
        let _progress = Progress::indeterminate("Loading scene description");
        let f = BufReader::new(File::open(config_path)?);
        serde_yaml::from_reader(f)?
    };

    // Render
    let state = match state_file {
        None => None,
        Some(file) => {
            let reader = BufReader::new(File::open(file)?);
            Some(bincode::deserialize_from(reader)?)
        }
    };
    let mut renderer = Renderer::new(running, &integrator, state);
    let state = renderer.render();

    // Save Results
    {
        let img_save_path = config_path.with_extension("exr");
        let _progress = Progress::indeterminate("Saving rendered image");
        state.img.save_exr(img_save_path.to_str().unwrap())?;
    }
    {
        let state_save_path = config_path.with_extension("state");
        let _progress = Progress::indeterminate("Saving render state");
        let f = BufWriter::new(File::create(state_save_path)?);
        bincode::serialize_into(f, state)?;
    }

    Ok(())
}
