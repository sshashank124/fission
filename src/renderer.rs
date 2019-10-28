use std::time::Instant;

use crate::image::Image;
use crate::integrator::{Integrator, Normals};
use crate::sampler::{Sampler, Uniform};
use crate::scene::Scene;
use crate::types::*;


pub struct Renderer<I = Normals, S = Uniform> where I: Integrator,
                                                    S: Sampler {
    integrator: I,
    sampler: S,
    scene: Scene,
}

impl Renderer {
    pub fn default(scene: Scene) -> Renderer {
        Renderer::new(Normals::new(), Uniform::new(), scene)
    }
}

impl<I, S> Renderer<I, S> where I: Integrator,
                                S: Sampler {
    pub fn new(integrator: I, sampler: S, scene: Scene) -> Renderer<I, S> {
        Renderer {
            integrator,
            sampler,
            scene,
        }
    }

    pub fn render(&self) -> Image {
        let camera = &self.scene.camera;
        let mut img = Image::new(camera.resolution());

        let t = Instant::now();
        for pixel in img.pixels() {
            let mut sampler = self.sampler.clone_seeded(img.flatten(pixel));
            let mut c = Color::BLACK;
            for _ in 0..1 {
                let primary_ray = camera.ray_at(pixel + sampler.gen_2d());
                c += self.integrator.sample(&self.scene, primary_ray);
            }
            img[pixel] = c / 1.;
        }
        println!("{:?}", t.elapsed());

        img
    }
}
