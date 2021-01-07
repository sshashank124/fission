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
use util::progress::Progress;

pub fn load_from_file(scene_file: impl AsRef<Path>,
                      state_file: Option<impl AsRef<Path>>)
    -> anyhow::Result<(Renderer, Arc<AtomicBool>)>
{
    let running = Arc::new(AtomicBool::new(true));

    let integrator = {
        let _p = Progress::indeterminate("Loading scene description");
        let f = BufReader::new(File::open(scene_file)?);
        serde_yaml::from_reader(f)?
    };

    let state = if let Some(state_file) = state_file {
        let _p = Progress::indeterminate("Loading render state");
        let f = BufReader::new(File::open(state_file)?);
        Some(bincode::deserialize_from(f)?)
    } else { None };

    let renderer = Renderer::new(running.clone(), integrator, state);

    Ok((renderer, running))
}

pub fn save_to_file(scene_file: impl AsRef<Path>,
                    state: &RenderState) -> anyhow::Result<()> {
    let scene_file = scene_file.as_ref();
    {
        let _p = Progress::indeterminate("Saving rendered image");
        let img_save_path = scene_file.with_extension("exr");
        state.img.save_exr(img_save_path.to_str().unwrap())?;
    }
    {
        let _p = Progress::indeterminate("Saving render state");
        let state_save_path = scene_file.with_extension("state");
        let f = BufWriter::new(File::create(state_save_path)?);
        bincode::serialize_into(f, state)?;
    }
    Ok(())
}