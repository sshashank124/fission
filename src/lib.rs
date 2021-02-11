#![feature(try_trait)]

#![warn(clippy::all,
        clippy::cargo,
        clippy::nursery,
        clippy::pedantic)]

#![allow(clippy::cast_possible_truncation,
         clippy::cast_possible_wrap,
         clippy::cast_precision_loss,
         clippy::cast_sign_loss,
         clippy::missing_errors_doc,
         clippy::missing_panics_doc,
         clippy::multiple_crate_versions,
         clippy::must_use_candidate,
         clippy::non_ascii_literal,
         clippy::unreadable_literal)]

#![allow(clippy::cargo_common_metadata)]

    mod aggregate;
    mod bsdf;
    mod camera;
pub mod color;
pub mod image;
    mod light;
pub mod renderer;
    mod sampler;
    mod scene;
    mod shape;
    mod texture;
    mod tracer;
pub mod util;

pub use graphite;


use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::{Arc, atomic::AtomicBool};

use renderer::{Renderer, RenderState};
use util::{config, progress::Progress};

pub fn load_from_file<P1, P2>(scene_file: P1, state_file: Option<P2>)
    -> anyhow::Result<(Renderer, Arc<AtomicBool>)>
where P1: AsRef<Path>,
      P2: AsRef<Path>,
{
    config::set_scene_root_dir(&scene_file);

    let integrator = {
        let msg = format!("Loading scene description ({})", scene_file.as_ref().display());
        let _p = Progress::indeterminate(&msg);
        let f = BufReader::new(File::open(scene_file)?);
        serde_yaml::from_reader(f)?
    };

    let state = if let Some(state_file) = state_file {
        let msg = format!("Loading render state ({})", state_file.as_ref().display());
        let _p = Progress::indeterminate(&msg);
        let f = BufReader::new(File::open(state_file)?);
        Some(bincode::deserialize_from(f)?)
    } else { None };

    let running = Arc::new(AtomicBool::new(true));
    let renderer = Renderer::new(running.clone(), integrator, state);

    Ok((renderer, running))
}

pub fn save_to_file<P>(scene_file: P, state: &RenderState) -> anyhow::Result<()>
where P: AsRef<Path> {
    let scene_file = scene_file.as_ref();
    {
        let img_save_path = scene_file.with_extension("exr");
        let msg = format!("Saving rendered image ({})", img_save_path.display());
        let _p = Progress::indeterminate(&msg);
        state.img.save_exr(img_save_path.to_str().unwrap())?;
    }
    {
        let state_save_path = scene_file.with_extension("state");
        let msg = format!("Saving render state ({})", state_save_path.display());
        let _p = Progress::indeterminate(&msg);
        let f = BufWriter::new(File::create(state_save_path)?);
        bincode::serialize_into(f, state)?;
    }
    Ok(())
}
