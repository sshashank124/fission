use std::fs::File;
use std::io::Read;

use yaml_rust::{Yaml, YamlLoader};

use super::*;
use crate::bsdf::*;
use crate::camera::*;
use crate::geometry::*;
use crate::integrator::*;
use crate::light::*;
use crate::loader::obj;
use crate::sampler::*;
use crate::scene::*;
use crate::shape::*;
use crate::tracer::*;


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
        "ambient_occlusion" => {
            let rl = fo(&config["ray_length"]);
            let s = io(&config["samples"]);
            AmbientOcclusion::new(s, rl).into()
        },
        "direct" => Direct::new().into(),
        "normals" => Normals::new().into(),
        "silhouette" => Silhouette::new().into(),
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

enum Element {
    Shape(Shape),
    Light(Light),
}

impl From<Shape> for Element
{ #[inline(always)] fn from(s: Shape) -> Self { Self::Shape(s) } }

impl From<Light> for Element
{ #[inline(always)] fn from(s: Light) -> Self { Self::Light(s) } }

fn load_scene(config: &Yaml) -> Res<Scene> {
    let camera = load_camera(&config["camera"])
                     .with_msg("failed to parse camera config")?;

    let (shapes, lights): (Vec<Element>, Vec<Element>)
        = v(&config["elements"], "missing list of elements")?
                      .iter()
                      .flat_map(|c| load_element(c))
                      .partition(|e| match e { Element::Shape(_) => true,
                                               _ => false });

    let shapes = shapes.into_iter().map(|e| match e { Element::Shape(s) => s,
                                                      _ => unreachable!() })
                       .collect::<Vec<_>>();

    let lights = lights.into_iter().map(|e| match e { Element::Light(l) => l,
                                                      _ => unreachable!() })
                       .collect::<Vec<_>>();

    Ok(Scene::new(shapes, lights, camera))
}

fn load_element(config: &Yaml) -> Res<Element> {
    let to_world = load_transforms(&config["transforms"])?;
    let bsdf = load_bsdf(&config["bsdf"])?;

    Ok(match s(&config["type"], "missing element type")? {
        "mesh" => {
            let filename = s(&config["obj"], "malformed filename")?;
            Shape::new(obj::load_from_file(filename, to_world)?.into(),
                       bsdf).into()
        },
        "sphere" => {
            let c = P(f3(&config["center"])
                        .with_msg("failed to parse sphere center")?);
            let r = f(&config["radius"], "failed to parse sphere radius")?;
            Shape::new(Sphere::new(c, r).into(),
                       bsdf).into()
        },
        "pointlight" => {
            let power = Color(f3(&config["power"])
                            .with_msg("failed to parse light power")?);
            let pos = P(f3(&config["position"])
                          .with_msg("failed to parse light position")?);
            Light::new(Point::new(power, pos).into()).into()
        },
        _ => return Err("unknown element type".into()),
    })
}

fn load_bsdf(config: &Yaml) -> Res<BSDF> {
    if config.is_badvalue() { return Ok(BSDF::ZERO); }

    Ok(match s(&config["type"], "missing bsdf type")? {
        "diffuse" => {
            let albedo = Color(f3(&config["albedo"])
                             .with_msg("failed to parse albedo")?);
            Diffuse::new(albedo).into()
        },
        _ => return Err("unknown bsdf type".into()),
    })
}

fn load_camera(config: &Yaml) -> Res<Camera> {
    let res = v(&config["resolution"], "missing resolution")?;
    if res.len() != 2 { return Err("malformed resolution".into()); }
    let res = A2(i(&res[0], "malformed width")?,
                 i(&res[1], "malformed height")?);

    let to_world = load_transforms(&config["transforms"])?;

    let model: CameraType = match s(&config["type"], "missing camera type")? {
        "perspective" => load_perspective_camera(config)?.into(),
        _ => return Err("unknown camera type".into()),
    };

    Ok(Camera::new(model, res, to_world))
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

#[inline(always)] fn f(f: &Yaml, msg: &str) -> Res<F>
{ fo(f).ok_or_else(|| msg.into()) }

#[inline(always)] fn i(i: &Yaml, msg: &str) -> Res<I>
{ io(i).ok_or_else(|| msg.into()) }

#[inline(always)] fn s<'a>(s: &'a Yaml, msg: &str) -> Res<&'a str>
{ so(s).ok_or_else(|| msg.into()) }

#[inline(always)] fn v<'a>(v: &'a Yaml, msg: &str) -> Res<&'a Vec<Yaml>>
{ vo(v).ok_or_else(|| msg.into()) }

#[inline(always)] fn fo(f: &Yaml) -> Option<F> { f.as_f64().map(|f| f as F) }

#[inline(always)] fn io(i: &Yaml) -> Option<I> { i.as_i64().map(|i| i as I) }

#[inline(always)] fn so(s: &Yaml) -> Option<&'_ str> { s.as_str() }

#[inline(always)] fn vo<'a>(v: &'a Yaml) -> Option<&'a Vec<Yaml>>
{ v.as_vec() }
