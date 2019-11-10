use std::time::Instant;

use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::image::{Block, Image, Pixel};
use crate::integrator::{Integrator, Normals};
use crate::sampler::{Sampler, Uniform};
use crate::scene::Scene;
use crate::types::*;


const SAMPLES_PER_PIXEL: I = 32;
const BLOCK_SIZE: I2 = P2(32, 32);

pub struct Renderer<Ig = Normals,
                     S = Uniform> where Ig: Integrator,
                                         S: Sampler {
    integrator: Ig,
    sampler: S,
    scene: Scene,
    spp: I,
}

impl Renderer {
    pub fn default(scene: Scene) -> Renderer {
        Renderer::new(Normals::new(), Uniform::new(), scene, SAMPLES_PER_PIXEL)
    }
}

impl<Ig, S> Renderer<Ig, S> where Ig: Integrator + Sync,
                                   S: Sampler + Sync {
    pub fn new(integrator: Ig, sampler: S, scene: Scene, spp: I)
            -> Renderer<Ig, S> {
        Renderer { integrator, sampler, scene, spp }
    }

    pub fn render(&self) -> Image {
        let camera = &self.scene.camera;

        let mut img = Image::new(camera.resolution());

        let render_view = #[inline(always)] |i| {
            let render_block = #[inline(always)] |mut block: Block| {
                let mut sampler = self.sampler.clone_seeded(block.flat_pos()
                                                            * i);

                let render_pixel = #[inline(always)] |mut pixel: Pixel| {
                    let sample_point = pixel.pos + sampler.gen_2d();
                    let color = self.integrator
                                    .sample(&self.scene,
                                            &mut sampler,
                                            camera.ray_at(sample_point));
                    *pixel += color / self.spp;
                };

                block.pixels().for_each(render_pixel);
            };

            img.as_block().blocks(BLOCK_SIZE)
                          .par_bridge()
                          .for_each(render_block);
        };

        let t = Instant::now();
        (0..self.spp).for_each(render_view);
        println!("{:?}", t.elapsed());

        img
    }
}
