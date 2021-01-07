use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

#[allow(clippy::wildcard_imports)]
use graphite::*;
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::image::{Block, Image};
use crate::sampler::Sampler;
use crate::scene::Scene;
use crate::tracer::Tracer;
use crate::util::progress::Progress;

#[derive(Debug, Deserialize)]
pub struct Integrator {
    tracer:  Tracer,
    sampler: Sampler,
    scene:   Scene,
    passes:  I,
}

pub struct Renderer {
    pub state:      RenderState,
        running:    Arc<AtomicBool>,
        integrator: Integrator,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RenderState {
    pub img:  Image,
    pub pass: I,
}

impl Renderer {
    pub fn new(running: Arc<AtomicBool>, integrator: Integrator,
               state: Option<RenderState>) -> Self {
        Self {
            state: state.unwrap_or_else(|| RenderState {
                img: Image::new(integrator.scene.camera.resolution),
                pass: 0,
            }), integrator, running
        }
    }

    pub fn render(self) -> crossbeam_channel::Receiver<RenderState> {
        let (frame_tx, frame_rx) = crossbeam_channel::unbounded();

        thread::spawn(move || {
            let Integrator { tracer, sampler, scene, passes } = self.integrator;
            let mut state = self.state;

            let mut progress = Progress::new("Rendering", state.pass, passes);
            while state.pass < passes {
                if !self.running.load(Ordering::SeqCst) {
                    progress.cancel();
                    break
                }

                state.img += state.img.rect.chunks().par_bridge().map(|rect| {
                    let mut sampler = sampler.for_rect(state.pass, &rect);

                    Block::from_iter(rect, rect.positions().map(|pos| {
                        sampler.prepare_for_pixel(pos);

                        let pos = F2::of(pos) + sampler.next_2d();
                        let ray = scene.camera.ray_at(pos, &mut sampler);

                        (pos, tracer.trace(&scene, &mut sampler, ray))
                    }))
                }).fold_with(Image::new(scene.camera.resolution), |mut img, b| {
                    img += b; img
                }).sum();

                state.pass += 1;
                let _ = frame_tx.send(state.clone());
                progress.update();
            }
        });

        frame_rx
    }
}
