use std::io::{stdout, Write};
use std::time::Instant;

use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::camera::*;
use crate::image::{Block, Image, Pixel};
use crate::integrator::*;
use crate::sampler::*;
use crate::scene::Scene;
use crate::types::*;


const BLOCK_SIZE: I2 = P2(32, 32);

pub struct Renderer {
    integrator: Integrator,
    sampler: Sampler,
    scene: Scene,
    spp: I,
}

impl Renderer {
    pub fn new(integrator: Integrator, sampler: Sampler, scene: Scene, spp: I)
            -> Renderer {
        Renderer { integrator, sampler, scene, spp }
    }

    pub fn render(&self) -> Image {
        let camera = &self.scene.camera;

        let mut img = Image::new(camera.resolution);

        let render_view = |i| {
            print!("\rRENDERING ... [{:4}/{:4}]", i + 1, self.spp);
            stdout().flush().unwrap();
            let render_block = |mut block: Block| {
                let mut sampler = self.sampler.clone_seeded(block.flat_pos()
                                                            + i);

                let render_pixel = |mut pixel: Pixel| {
                    let sample_point = pixel.pos + sampler.gen_2d();
                    let ray = camera.ray_at(sample_point, &mut sampler);
                    let color = self.integrator.sample(&self.scene,
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
        (0..self.spp).for_each(render_view);
        img.as_block().pixels().for_each(|mut pixel| *pixel /= self.spp);
        println!("\rRendering ... DONE ({:?})", t.elapsed());

        img
    }
}
