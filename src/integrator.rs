use std::sync::mpsc;

use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::image::*;
use crate::prelude::*;
use crate::sampler::*;
use crate::scene::*;
use crate::tracer::*;
use crate::util::{Progress, threaded};

#[derive(Debug, Deserialize)]
pub struct Integrator {
    tracer:  Tracer,
    sampler: Sampler,
    scene:   Scene,
}

impl Integrator {
    pub fn render(&self) -> Image {
        let mut img = self.scene.camera.new_image();
        let img_rect = img.rect;

        let (block_tx, block_rx) = mpsc::channel();
        threaded::run(move || {
            let mut progress = Progress::new("Rendering", self.sampler.spp);
            for i in 0..self.sampler.spp {
                img_rect.chunks().par_bridge().map(|rect| {
                    let mut sampler = self.sampler.for_rect(i, &rect);

                    Block::from_iter(rect, rect.positions().map(|pos| {
                        sampler.prepare_for_pixel(pos);

                        let pos = F2::from(pos) + sampler.next_2d();
                        let ray = self.scene.camera.ray_at(pos, &mut sampler);

                        (pos, self.tracer.trace(&self.scene, &mut sampler, ray))
                    }))
                }).for_each_with(block_tx.clone(), |tx, b| tx.send(b).unwrap());
                progress.update();
            }
        });

        block_rx.iter().for_each(|block| img += block);
        img
    }
}
