use std::io::{stdout, Write};
use std::time::Instant;

use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::camera::*;
use crate::image::*;
use crate::sampler::*;
use crate::scene::*;
use crate::tracer::*;
use crate::types::*;


pub struct Integrator {
    tracer: Tracer,
    sampler: Sampler,
    scene: Scene,
}

impl Integrator {
    pub fn new(tracer: Tracer, sampler: Sampler, scene: Scene)
            -> Self {
        Self { tracer, sampler, scene }
    }

    pub fn render(&self) -> Image {
        let camera = &self.scene.camera;

        let mut img = camera.new_image();

        let render_view = |i| {
            print!("\rRENDERING ... [{:4}/{:4}]", i + 1, self.sampler.spp);
            stdout().flush().unwrap();

            let render_block = |mut block: Block| {
                let pixels = block.pixels();
                let mut sampler = self.sampler.clone_seeded((i, &block));

                let render_pixel = |pos: I2| {
                    sampler.prepare_for_pixel(pos);

                    let sample_pos = F2::from(pos) + sampler.next_2d();
                    let ray = camera.ray_at(sample_pos, &mut sampler);
                    let color = self.tracer.trace(&self.scene,
                                                  &mut sampler,
                                                  ray);

                    block.put(sample_pos, color);
                };

                pixels.for_each(render_pixel);
            };

            img.as_block().blocks()
                          .par_bridge()
                          .for_each(render_block);
        };

        let t = Instant::now();
        (0..self.sampler.spp).for_each(render_view);
        println!("\rRendering ... DONE ({:?})", t.elapsed());

        img
    }
}
