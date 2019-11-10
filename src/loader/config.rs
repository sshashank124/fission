use std::fs::File;
use std::io::Read;

use yaml_rust::{Yaml, YamlLoader};

use crate::camera::*;
use crate::geometry::*;
use crate::integrator::*;
use crate::loader::obj;
use crate::renderer::*;
use crate::sampler::*;
use crate::scene::*;
use crate::structure::*;
use crate::util::*;


pub fn load_from_file(filename: &str) -> Res<Renderer> {
    let mut f = File::open(filename).with_msg("Error opening config file")?;
    let mut config_str = String::new();
    f.read_to_string(&mut config_str).with_msg("Error reading config file")?;

    let doc = &YamlLoader::load_from_str(&config_str)
                          .with_msg("Invalid YAML syntax")?[0];
    load_from_doc(doc)
}

fn load_from_doc(config: &Yaml) -> Res<Renderer> {
    let scene = load_scene(&config["scene"])
                    .with_msg("Failed to parse scene config")?;

    let config = &config["renderer"];

    let spp = config["samples_per_pixel"].as_i64()
                  .ok_or("missing samples_per_pixel")? as I;

    let integrator = load_integrator(&config["integrator"])
                         .with_msg("failed to parse integrator config")?;

    let sampler = load_sampler(&config["sampler"])
                      .with_msg("failed to parse sampler config")?;

    Ok(Renderer::new(integrator, sampler, scene, spp))
}

fn load_integrator(config: &Yaml) -> Res<Integrator> {
    Ok(match config["type"].as_str().ok_or("missing integrator type")? {
        "silhouette" => Silhouette::new().into(),
        "normals" => Normals::new().into(),
        "av" => {
            let ray_len = config["ray_length"].as_f64()
                              .ok_or("missing ray_length")? as F;
            AverageVisibility::new(ray_len).into()
        },
        _ => return Err("unknown integrator type".into()),
    })
}

fn load_sampler(config: &Yaml) -> Res<Sampler> {
    Ok(match config["type"].as_str().ok_or("missing sampler type")? {
        "uniform" => Uniform::new().into(),
        _ => return Err("unknown sampler type".into()),
    })
}

fn load_scene(config: &Yaml) -> Res<Scene> {
    let camera = load_camera(&config["camera"])
                     .with_msg("failed to parse camera config")?;

    let structures = load_structures(&config["structures"])
                         .with_msg("failed to parse structures config")?;

    Ok(Scene::new(camera, structures))
}

fn load_structures(config: &Yaml) -> Res<Structure> {
    let structures: Result<Vec<_>, _> =
        config.as_vec().ok_or("missing list of structures")?
                       .iter()
                       .map(|c| load_structure(c))
                       .collect();
    let structures = structures.with_msg("failed to parse structures")?;

    Ok(Structure::new(BVH::new(structures), T::I))
}

fn load_structure(config: &Yaml) -> Res<Structure> {
    let to_world = load_transforms(&config["transforms"])?;

    let s = match config["type"].as_str().ok_or("missing structure type")? {
        "mesh" => {
            let filename = config["obj"].as_str().ok_or("malformed filename")?;
            StructureType::from(obj::load_from_file(filename)?)
        },
        "sphere" => {
            let r = config["radius"].as_f64().ok_or("missing radius")? as F;
            StructureType::from(Sphere::new(r))
        },
        _ => return Err("unknown structure type".into()),
    };

    Ok(Structure::new(s, to_world))
}

fn load_camera(config: &Yaml) -> Res<Camera> {
    let res = config["resolution"].as_vec()
                  .ok_or("missing resolution")?;
    if res.len() != 2 { return Err("malformed resolution".into()); }
    let res = P2(res[0].as_i64().ok_or("malformed width")? as I,
                 res[1].as_i64().ok_or("malformed height")? as I);

    let to_world = load_transforms(&config["transforms"])?;

    let model = match config["type"].as_str().ok_or("missing camera type")? {
        "perspective" => CameraType::from(load_perspective_camera(config)?),
        _ => return Err("unknown camera type".into()),
    };

    Ok(Camera::new(model, res, to_world))
}

fn load_perspective_camera(config: &Yaml) -> Res<Perspective> {
    let fov = config["fov"].as_f64().ok_or("missing fov")? as F;
    Ok(Perspective::new(fov))
}

fn load_transforms(config: &Yaml) -> Res<T> {
    match config.as_vec() {
        None => Ok(T::I),
        Some(transforms) =>
            transforms.iter().try_fold(T::I, |acc, t| {
                let t = match t.as_hash() {
                    None => Ok(T::I),
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
