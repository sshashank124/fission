use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[allow(clippy::wildcard_imports)]
use graphite::*;
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::image::{Block, Image};
use crate::sampler::Sampler;
use crate::scene::Scene;
use crate::tracer::Tracer;
use crate::util::{progress::Progress, threaded};

#[derive(Debug, Deserialize)]
pub struct Integrator {
    tracer:  Tracer,
    sampler: Sampler,
    scene:   Scene,
    passes:  I,
}

pub struct Renderer<'a> {
    state:      RenderState,
    running:    Arc<AtomicBool>,
    integrator: &'a Integrator,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RenderState {
    pub img:  Image,
        pass: I,
}

impl<'a> Renderer<'a> {
    pub fn new(running: Arc<AtomicBool>, integrator: &'a Integrator,
                      state: Option<RenderState>) -> Self {
        Self {
            integrator, running,
            state: state.unwrap_or_else(|| RenderState {
                img: Image::new(integrator.scene.camera.resolution),
                pass: 0,
            })
        }
    }

    pub fn render(&mut self) -> &RenderState {
        let (block_tx, block_rx) = crossbeam::channel::unbounded();

        threaded::run(|| {
            let block_tx = block_tx;
            let Integrator { tracer, sampler, scene, passes } = self.integrator;

            let mut progress = Progress::new("Rendering",
                                             self.state.pass, *passes);
            while self.state.pass < *passes {
                if !self.running.load(Ordering::SeqCst) {
                    progress.cancel();
                    break
                }

                self.state.img.rect.chunks().par_bridge().map(|rect| {
                    let mut sampler = sampler.for_rect(self.state.pass, &rect);

                    Block::from_iter(rect, rect.positions().map(|pos| {
                        sampler.prepare_for_pixel(pos);

                        let pos = F2::from(pos) + sampler.next_2d();
                        let ray = scene.camera.ray_at(pos, &mut sampler);

                        (pos, tracer.trace(scene, &mut sampler, ray))
                    }))
                }).for_each_with(&block_tx, |tx, b| tx.send(b).unwrap());
                progress.update();
                self.state.pass += 1;
            }
        });

        block_rx.iter().for_each(|block| self.state.img += block);
        &self.state
    }
}