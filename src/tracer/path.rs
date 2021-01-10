#[allow(clippy::wildcard_imports)]
use graphite::*;
use serde::Deserialize;

use crate::color::Color;
use crate::sampler::Sampler;
use crate::scene::Scene;
use crate::util::either::Either;

use super::direct;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Path {
    depth: I2,
    rr_tp: F,
}

impl Path {
    #[inline]
    pub fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
        let init = (Color::ZERO, Color::ONE, ray, scene.intersect(ray), true);
        if init.3.is_none() { return scene.lenv(&ray) }

        match (0..self.depth[1]).try_fold(init,
            |(mut li, mut tp, ray, its, spec), depth|
                its.map_or_else(move || Either::L(li + tp * scene.lenv(&ray)),
                                |its| {
                    if spec { li += tp * its.le(ray); }
                    let (ld, lb, ray, its, spec) = direct::li(scene, sampler,
                                                              &its, &ray);
                    li += tp * ld;
                    tp *= lb;

                    if its.is_none() {
                        return Either::L(li + tp * scene.lenv(&ray))
                    }

                    if depth > self.depth[0] {
                        let q = F::min(tp.max_channel(), self.rr_tp);
                        if sampler.rng() > q { return Either::L(li) }
                        tp /= q;
                    }

                    Either::R((li, tp, ray, its, spec))
                }
            )
        ) { Either::L(li) | Either::R((li, _, _, _, _)) => li }
    }
}

impl Default for Path
{ fn default() -> Self { Self { depth: A2(10, 20), rr_tp: 1. } } }
