use std::fs::File;
use std::io::Read;

use yaml_rust::{Yaml, YamlLoader};

use crate::aggregate::*;
use crate::camera::*;
use crate::filter::*;
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
    Ok(match s(&config["type"], "missing tracer type")? {
        "silhouette" => Silhouette::new().into(),
        "normals" => Normals::new().into(),
        "av" => {
            let rl = f(&config["ray_length"], "missing ray_length")?;
            AverageVisibility::new(rl).into()
        },
        _ => return Err("unknown tracer type".into()),
    })
}

fn load_sampler(config: &Yaml) -> Res<Sampler> {
    let st: SamplerType =
        match s(&config["type"], "missing sampler type")? {
            "independent" => Independent::new().into(),
            "sobol" => Sobol::new().into(),
            _ => return Err("unknown sampler type".into()),
    };

    let spp = i(&config["samples_per_pixel"],
                      "missing samples_per_pixel")?;

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
    let shapes = v(config, "missing list of shapes")?
                     .iter()
                     .flat_map(|c| load_mesh(c))
                     .flatten().collect::<Vec<_>>();

    Ok(Mesh::new(shapes))
}

fn load_mesh(config: &Yaml) -> Res<Vec<Triangle>> {
    let to_world = load_transforms(&config["transforms"])?;

    let filename = s(&config["obj"], "malformed filename")?;
    obj::load_from_file(filename, to_world)
}

fn load_camera(config: &Yaml) -> Res<Camera> {
    let res = v(&config["resolution"], "missing resolution")?;
    if res.len() != 2 { return Err("malformed resolution".into()); }
    let res = P2(i(&res[0], "malformed width")?,
                 i(&res[1], "malformed height")?);

    let to_world = load_transforms(&config["transforms"])?;

    let model: CameraType = match s(&config["type"], "missing camera type")? {
        "perspective" => load_perspective_camera(config)?.into(),
        _ => return Err("unknown camera type".into()),
    };

    let rfilter_config = &config["reconstruction_filter"];
    let rfilter = if !rfilter_config.is_badvalue() {
        Some(load_rfilter(rfilter_config)?)
    } else { None };

    Ok(Camera::new(model, res, rfilter, to_world))
}

fn load_rfilter(config: &Yaml) -> Res<ReconstructionFilter> {
    let radius = f(&config["radius"], "missing filter radius")?;

    Ok(match s(&config["type"], "missing filter type")? {
        "square" => Square::new(radius).into(),
        "gaussian" => {
            let stdev = f(&config["stdev"], "missing gaussian stdev")?;
            Gaussian::new(radius, stdev).into()
        },
        _ => return Err("unknown filter type".into()),
    })
}

fn load_perspective_camera(config: &Yaml) -> Res<Perspective> {
    let fov = f(&config["fov"], "missing fov")?;
    let lens_radius = fo(&config["lens_radius"]);
    let focal_distance = fo(&config["focal_distance"]);
    Ok(Perspective::new(fov, lens_radius, focal_distance))
}

fn load_transforms(config: &Yaml) -> Res<T> {
    match vo(config) {
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
    Ok(match s(&ttype, "expected transform name")? {
        "scale" => {
            let s = f3(config)
                        .with_msg("failed to parse scaling amounts")?;
            T::scale(s)
        },
        "rotate" => {
            let axis = f3(&config["axis"])
                           .with_msg("failed to parse rotation angle")?;
            let theta = f(&config["angle"], "failed to parse rotation angle")?;
            T::rotate(axis, theta)
        },
        "translate" => {
            let t = f3(config)
                        .with_msg("failed to parse translation amounts")?;
            T::translate(t)
        },
        "look_at" => {
            let origin = P(f3(&config["origin"])
                             .with_msg("failed to parse origin")?);
            let target = P(f3(&config["target"])
                             .with_msg("failed to parse target")?);
            let up = V(f3(&config["up"])
                         .with_msg("failed to parse up-vector")?);
            T::look_at(origin, target, up)
        },
        _ => return Err("unknown transform type".into()),
    })
}

#[inline(always)]
fn f3(vec: &Yaml) -> Res<F3> {
    let v = v(vec, "expected 3d vector")?;
    if v.len() != 3 { return Err("malformed 3d vector".into()); }
    Ok(A3(f(&v[0], "malformed X value")?,
          f(&v[1], "malformed Y value")?,
          f(&v[2], "malformed Z value")?))
}

#[inline(always)]
fn f(f: &Yaml, msg: &str) -> Res<F> { fo(f).ok_or(msg.into()) }

#[inline(always)]
fn i(i: &Yaml, msg: &str) -> Res<I> { io(i).ok_or(msg.into()) }

#[inline(always)]
fn s<'a>(s: &'a Yaml, msg: &str) -> Res<&'a str> {
    s.as_str().ok_or(msg.into())
}

#[inline(always)]
fn v<'a>(v: &'a Yaml, msg: &str) -> Res<&'a Vec<Yaml>> {
    vo(v).ok_or(msg.into())
}

#[inline(always)]
fn fo(f: &Yaml) -> Option<F> { f.as_f64().map(|f| f as F) }

#[inline(always)]
fn io(i: &Yaml) -> Option<I> { i.as_i64().map(|i| i as I) }

#[inline(always)]
fn vo<'a>(v: &'a Yaml) -> Option<&'a Vec<Yaml>> { v.as_vec() }
