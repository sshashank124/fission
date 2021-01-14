#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

use crate::aggregate::bvh::BVH;
use crate::camera::Camera;
use crate::color::Color;
use crate::light::Light;
use crate::shape::{Intersectable, Shape, intersection::Its};
use crate::util::{dpdf::DiscretePDF, pdf::PDF};

#[derive(Debug, Deserialize)]
#[serde(from="SceneConfig")]
pub struct Scene {
    pub camera:      Camera,
        shapes:      BVH<&'static Shape>,
    pub lights:      Box<[&'static Light]>,
        lights_dpdf: DiscretePDF,
        env:         Option<&'static Light>,
}

impl Scene {
    #[inline] pub fn intersects(&self, r: R) -> bool { self.shapes.intersects(r) }
    #[inline] pub fn intersect(&self, r: R) -> Option<Its> { self.shapes.intersect(r) }

    #[inline] pub fn sample_random_light(&self, its: &Its,
                                         mut s: F2) -> (PDF<Color>, R) {
        let (idx, prob) = self.lights_dpdf.sample(&mut s[0]);
        let (l_light, sray) = self.lights[idx].sample(its, s);
        (l_light.scale(prob), sray)
    }

    #[inline] pub fn lenv(&self, ray: &R) -> Color
    { self.env.as_ref().map_or(Color::ZERO, |light| light.eval_env(ray)) }
}


#[derive(Debug, Deserialize)]
struct SceneConfig {
    camera:   Camera,
    elements: Vec<Element>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Element {
    Shape(Shape),
    Light(Light),
}

impl From<SceneConfig> for Scene {
    fn from(sc: SceneConfig) -> Self {
        let mut shapes = vec![];
        let mut lights = vec![];
        for elem in sc.elements {
            match elem {
                Element::Shape(s) => {
                    let emitter = s.emission.is_some();
                    let s: &Shape = Box::leak(Box::new(s));
                    if emitter {
                        let l: &Light = Box::leak(Box::new(s.into()));
                        lights.push(l);
                    }
                    shapes.push(s);
                },
                Element::Light(l) => lights.push(Box::leak(Box::new(l))),
            }
        }
        let shapes = BVH::new(shapes);
        let lights_dpdf = DiscretePDF::new(&lights, |light| light.power());
        let env = lights.iter().copied().find(|light| light.is_env_light());
        Self { shapes, camera: sc.camera,
               lights: lights.into_boxed_slice(), lights_dpdf, env }
}
}
