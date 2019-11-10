#![feature(stmt_expr_attributes)]
#![feature(test)]

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
mod warp;

use camera::{Camera, Perspective};
use geometry::*;
use integrator::AverageVisibility;
use loader::obj;
use renderer::Renderer;
use sampler::Uniform;
use scene::Scene;
use structure::Structure;
use util::*;


fn main() -> Res<()> {
    print!("Loading assets ... ");
    let ajax = obj::load_from_file("obj/ajax.obj")
                   .with_msg("Failed to load OBJ")?;

    let plane = obj::load_from_file("obj/plane.obj")
                    .with_msg("Failed to load OBJ")?;
    println!("DONE");

    print!("Creating scene ... ");
    let ajax_st = Structure::new(ajax, T::I);
    let plane_st = Structure::new(plane, T::scale(A3(100., 1., 100.)));
    let root = Structure::new(vec![ajax_st, plane_st], T::I);

    let camera = Camera::new(Perspective::new(30.), P2(768, 768),
                             T::look_at(P::p(-65.6055, 47.5762, 24.3583),
                                        P::p(-64.8161, 47.2211, 23.8576),
                                        V::v(0.299858, 0.934836, -0.190177)));
    let scene = Scene::new(camera, root);
    println!("DONE");

    print!("Initializing renderer ... ");
    let integrator = AverageVisibility::new(10.);
    let sampler = Uniform::new();
    let renderer = Renderer::new(integrator, sampler, scene, 1024);
    println!("DONE");

    let image = renderer.render();

    print!("Saving rendered image ... ");
    image.save_exr("test.exr").with_msg("Saving image failed")?;
    println!("DONE");

    Ok(())
}
