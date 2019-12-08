use crate::camera::*;
use crate::core::*;
use crate::image::*;
use crate::sampler::*;
use crate::scene::*;
use crate::tracer::*;


pub struct Integrator {
    tracer: Tracer,
    sampler: Sampler,
    scene: Scene,
}

impl Integrator {
    pub fn new(tracer: Tracer, sampler: Sampler, scene: Scene) -> Self
    { Self { tracer, sampler, scene } }

    pub fn render(&self) -> Image {
        let mut img = self.scene.camera.new_image();

        let mut progress = Progress::new("RENDERING", Some(self.sampler.spp));
        for i in 0..self.sampler.spp {
            img.as_block().blocks().parallelize().for_each(|mut block| {
                let mut sampler = self.sampler.clone_seeded((i, &block));

                for pos in block.pixels() {
                    sampler.prepare_for_pixel(pos);

                    let pos = F2::from(pos) + sampler.next_2d();
                    let ray = self.scene.camera.ray_at(pos, &mut sampler);
                    let color = self.tracer.trace(&self.scene,
                                                  &mut sampler,
                                                  ray);
                    block.put(pos, color);
                }
            });
            progress.update();
        }
        progress.finish();

        img
    }
}
