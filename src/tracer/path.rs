#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

use crate::color::Color;
use crate::sampler::Sampler;
use crate::scene::Scene;

use super::direct;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Path {
    depth: I2,
    rr_tp: F,
}

impl Path {
    #[inline] pub fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
        let mut state = direct::BounceInfo { l: Color::ZERO, tp: Color::ONE,
                                             its: scene.intersect(ray), ray, spec: true };

        for depth in 0..self.depth[1] {
            let its = match state.its {
                None => { state.l += state.tp * scene.lenv(&state.ray); break }
                Some(its) => its
            };

            if state.spec { state.l += state.tp * its.l_emit(state.ray); }

            let frame = its.to_world();
            let wo = frame / -state.ray.d;

            state.l += state.tp * direct::l_light(scene, &its, wo, frame, sampler.next_2d());

            if depth > self.depth[0] {
                let q = F::min(state.tp.max_channel(), self.rr_tp);
                if sampler.rng() > q { break }
                state.tp /= q;
            }

            let bounce = match direct::l_mat(scene, &its, wo, frame, sampler.next_2d()) {
                None => break,
                Some(bounce) => bounce,
            };

            state.l += state.tp * bounce.l;
            state.tp *= bounce.tp;
            state.spec = bounce.spec;
            state.ray = bounce.ray;
            state.its = bounce.its;
        }

        state.l
    }
}

impl Default for Path
{ fn default() -> Self { Self { depth: A2(10, 20), rr_tp: 1. } } }
