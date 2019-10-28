mod camera;
mod geometry;
mod image;
mod integrator;
mod loader;
mod renderer;
mod sampler;
mod scene;
mod solver;
mod structure;
mod types;
mod util;

use camera::Camera;
use geometry::*;
use loader::obj;
use renderer::Renderer;
use scene::Scene;
use structure::Structure;
use util::*;


fn main() -> Res<()> {
    let mesh = obj::load_from_file("teapot.obj")
                   .with_msg("Failed to load OBJ")?;

    let root = Structure::new(mesh, T::translate(A3(0.,-2.,8.)));
    let scene = Scene::new(Camera::default(), root);

    let renderer = Renderer::default(scene);

    let image = renderer.render();

    image.save_exr("test.exr").with_msg("Saving image failed")?;

    Ok(())
}
