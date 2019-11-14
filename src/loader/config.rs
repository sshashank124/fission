use std::fs::File;
use std::io::Read;

use yaml_rust::{Yaml, YamlLoader};

use crate::aggregate::*;
use crate::camera::*;
use crate::geometry::*;
use crate::integrator::*;
use crate::loader::obj;
use crate::sampler::*;
use crate::scene::*;
use crate::tracer::*;
use crate::util::*;


pub fn load_from_file(filename: &str) -> Res<Integrator> {
    let mut f = File::open(filename).with_msg("Error opening config file")?;
    let mut config_str = String::new();
    f.read_to_string(&mut config_str).with_msg("Error reading config file")?;

    let doc = &YamlLoader::load_from_str(&config_str)
                          .with_msg("Invalid YAML syntax")?[0];
    load_from_doc(doc)
}

fn load_from_doc(config: &Yaml) -> Res<Integrator> {
    let scene = load_scene(&config["scene"])
                    .with_msg("Failed to parse scene config")?;

    let config = &config["integrator"];

    let tracer = load_tracer(&config["tracer"])
                         .with_msg("failed to parse tracer config")?;

    let sampler = load_sampler(&config["sampler"])
                      .with_msg("failed to parse sampler config")?;

    Ok(Integrator::new(tracer, sampler, scene))
}

fn load_tracer(config: &Yaml) -> Res<Tracer> {
    Ok(match config["type"].as_str().ok_or("missing tracer type")? {
        "silhouette" => Silhouette::new().into(),
        "normals" => Normals::new().into(),
        "av" => {
            let ray_len = config["ray_length"].as_f64()
                              .ok_or("missing ray_length")? as F;
            AverageVisibility::new(ray_len).into()
        },
        _ => return Err("unknown tracer type".into()),
    })
}

fn load_sampler(config: &Yaml) -> Res<Sampler> {
    let st: SamplerType =
        match config["type"].as_str().ok_or("missing sampler type")? {
            "independent" => Independent::new().into(),
            "sobol" => Sobol::new().into(),
            _ => return Err("unknown sampler type".into()),
    };

    let spp = config["samples_per_pixel"].as_i64()
                  .ok_or("missing samples_per_pixel")? as I;

    Ok(Sampler::new(st, spp))
}

fn load_scene(config: &Yaml) -> Res<Scene> {
    let camera = load_camera(&config["camera"])
                     .with_msg("failed to parse camera config")?;

    let shapes = load_shapes(&config["shapes"])
                         .with_msg("failed to parse shapes config")?;

    Ok(Scene::new(camera, shapes))
}

fn load_shapes(config: &Yaml) -> Res<Mesh> {
    let shapes = config.as_vec().ok_or("missing list of shapes")?
                                .iter()
                                .flat_map(|c| load_mesh(c))
                                .flatten().collect::<Vec<_>>();

    Ok(Mesh::new(shapes))
}

fn load_mesh(config: &Yaml) -> Res<Vec<Triangle>> {
    let to_world = load_transforms(&config["transforms"])?;

    let filename = config["obj"].as_str().ok_or("malformed filename")?;
    obj::load_from_file(filename, to_world)
}

fn load_camera(config: &Yaml) -> Res<Camera> {
    let res = config["resolution"].as_vec()
                  .ok_or("missing resolution")?;
    if res.len() != 2 { return Err("malformed resolution".into()); }
    let res = P2(res[0].as_i64().ok_or("malformed width")? as I,
                 res[1].as_i64().ok_or("malformed height")? as I);

    let to_world = load_transforms(&config["transforms"])?;

    let model: CameraType =
        match config["type"].as_str().ok_or("missing camera type")? {
            "perspective" => load_perspective_camera(config)?.into(),
            _ => return Err("unknown camera type".into()),
    };

    Ok(Camera::new(model, res, to_world))
}

fn load_perspective_camera(config: &Yaml) -> Res<Perspective> {
    let fov = config["fov"].as_f64().ok_or("missing fov")? as F;
    let lens_radius = config["lens_radius"].as_f64().map(|f| f as F);
    let focal_distance = config["focal_distance"].as_f64().map(|f| f as F);
    Ok(Perspective::new(fov, lens_radius, focal_distance))
}

fn load_transforms(config: &Yaml) -> Res<T> {
    match config.as_vec() {
        None => Ok(T::ONE),
        Some(transforms) =>
            transforms.iter().try_fold(T::ONE, |acc, t| {
                let t = match t.as_hash() {
                    None => Ok(T::ONE),
                    Some(t) => t.iter().map(load_transform).next().unwrap(),
                };
                t.map(|t| t * acc)
            }).with_msg("failed to parse transforms"),
    }
}

fn load_transform((ttype, config): (&Yaml, &Yaml)) -> Res<T> {
    Ok(match ttype.as_str().ok_or("expected transform name")? {
        "scale" => {
            let s = parse_f3(config)
                        .with_msg("failed to parse scaling amounts")?;
            T::scale(s)
        },
        "translate" => {
            let t = parse_f3(config)
                        .with_msg("failed to parse translation amounts")?;
            T::translate(t)
        },
        "look_at" => {
            let origin = P(parse_f3(&config["origin"])
                             .with_msg("failed to parse origin")?);
            let target = P(parse_f3(&config["target"])
                             .with_msg("failed to parse target")?);
            let up = V(parse_f3(&config["up"])
                         .with_msg("failed to parse up-vector")?);
            T::look_at(origin, target, up)
        },
        _ => return Err("unknown transform type".into()),
    })
}

fn parse_f3(vec: &Yaml) -> Res<F3> {
    let v = vec.as_vec().ok_or("expected 3d vector")?;
    if v.len() != 3 { return Err("malformed 3d vector".into()); }
    Ok(A3(v[0].as_f64().ok_or("malformed X value")? as F,
          v[1].as_f64().ok_or("malformed Y value")? as F,
          v[2].as_f64().ok_or("malformed Z value")? as F))
}
