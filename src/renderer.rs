use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;

use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::image::*;
use crate::prelude::*;
use crate::sampler::*;
use crate::scene::*;
use crate::tracer::*;
use crate::util::{Progress, threaded};

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

#[derive(Debug, Deserialize)]
pub struct Integrator {
    tracer:  Tracer,
    sampler: Sampler,
    scene:   Scene,
    passes:  I,
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
        let (block_tx, block_rx) = mpsc::channel();

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

                        (pos, tracer.trace(&scene, &mut sampler, ray))
                    }))
                }).for_each_with(block_tx.clone(), |tx, b| tx.send(b).unwrap());
                progress.update();
                self.state.pass += 1;
            }
        });

        block_rx.iter().for_each(|block| self.state.img += block);
        &self.state
    }
}
