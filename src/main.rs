#![feature(try_trait)]

#![warn(clippy::all,
        clippy::cargo,
        clippy::nursery,
        clippy::pedantic)]

#![allow(clippy::cast_possible_truncation,
         clippy::cast_possible_wrap,
         clippy::cast_sign_loss,
         clippy::find_map,
         clippy::inline_always,
         clippy::multiple_crate_versions,
         clippy::non_ascii_literal,
         clippy::unreadable_literal,
         clippy::wildcard_imports)]

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
    pub use anyhow::*;
    pub use graphite::*;
    pub use serde::{Deserialize, Serialize};
}

use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use prelude::*;
use renderer::{Integrator, Renderer};
use util::Progress;

fn main() -> Result<()> {
    let running = Arc::new(AtomicBool::new(true));

    let r = running.clone();
    ctrlc::set_handler(move || { r.store(false, Ordering::SeqCst); })?;

    // Parse Args
    let mut args = env::args();
    ensure!(args.next().is_some(), "I can't believe you've done this");

    let config_file = match args.next() {
        Some(arg) => arg,
        None => bail!("Usage: fission <scene_description.yaml> \
                       [render_progress.state]"),
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
    let state = state_file.map(File::open).transpose()?.map(BufReader::new)
                          .map(bincode::deserialize_from).transpose()?;
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
