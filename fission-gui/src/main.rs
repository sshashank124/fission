use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

use anyhow::{bail, ensure};

use fission_core::renderer::{Integrator, Renderer};
use fission_core::util::progress::Progress;

fn main() -> anyhow::Result<()> {
    let running = Arc::new(AtomicBool::new(true));

    let r = running.clone();

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
