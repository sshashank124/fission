use std::io::{stdout, Write};
use std::time::Instant;

use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::camera::*;
use crate::image::*;
use crate::sampler::*;
use crate::scene::*;
use crate::tracer::*;
use crate::types::*;


const BLOCK_SIZE: I2 = P2(16, 16);

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

        let mut img = Image::new(camera.resolution);

        let render_view = |i| {
            print!("\rRENDERING ... [{:4}/{:4}]", i + 1, self.sampler.spp);
            stdout().flush().unwrap();

            let render_block = |mut block: Block| {
                let mut sampler = self.sampler.clone_seeded((i, &block));

                let render_pixel = |mut pixel: Pixel| {
                    sampler.prepare_pixel(&pixel);

                    let sample_point = pixel.pos + sampler.next_2d();
                    let ray = camera.ray_at(sample_point, &mut sampler);
                    let color = self.tracer.trace(&self.scene,
                                                  &mut sampler,
                                                  ray);
                    *pixel += color;
                };

                block.pixels().for_each(render_pixel);
            };

            img.as_block().blocks(BLOCK_SIZE)
                          .par_bridge()
                          .for_each(render_block);
        };

        let t = Instant::now();
        (0..self.sampler.spp).into_iter().for_each(render_view);
        img.as_block().pixels().for_each(|mut pixel|  // TODO refactor
                                         *pixel /= self.sampler.spp);
        println!("\rRendering ... DONE ({:?})", t.elapsed());

        img
    }
}
