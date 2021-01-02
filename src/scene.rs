use crate::aggregate::BVH;
use crate::camera::Camera;
use crate::light::*;
use crate::prelude::*;
use crate::shape::*;
use crate::util::DiscretePDF;

#[derive(Debug, Deserialize)]
#[serde(from="SceneConfig")]
pub struct Scene {
    pub camera:      Camera,
        shapes:      BVH<Arc<Shape>>,
    pub lights:      Vec<Arc<Light>>,
    pub lights_dpdf: DiscretePDF,
        env:         Option<Arc<Light>>,
}

impl Scene {
    #[inline(always)] pub fn intersects(&self, r: R) -> bool
    { self.shapes.intersects(r) }

    #[inline(always)] pub fn intersect(&self, r: R) -> Option<Its>
    { self.shapes.intersect(r) }

    #[inline(always)]
    pub fn sample_random_light(&self, its: &Its, mut s: F2) -> (Color, R, F) {
        let idx = self.lights_dpdf.sample(&mut s[0]);
        self.lights[idx].sample(its, s)
    }

    #[inline(always)] pub fn lenv(&self, ray: &R) -> Color
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
                    let s = Arc::new(s);
                    if emitter { lights.push(s.clone().into()); }
                    shapes.push(s);
                },
                Element::Light(l) => lights.push(l),
            }
        }
        let shapes = BVH::new(shapes);
        let lights_dpdf = DiscretePDF::new(&lights, |_| Light::power());
        let lights = lights.into_iter().map(Arc::new).collect::<Vec<_>>();
        let env =
            lights.iter().find(|light| light.is_env_light()).map(Arc::clone);
        Self { shapes, camera: sc.camera, lights, lights_dpdf, env }
}
}
