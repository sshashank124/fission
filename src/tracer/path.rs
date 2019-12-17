use super::*;


pub struct Path {
    depth: I2,
    rr_tp: F,
}

impl Path {
    pub fn new(depth: Option<I2>, rr_tp: Option<F>) -> Self {
        let depth = depth.unwrap_or(A2(10, 20));
        let rr_tp = rr_tp.unwrap_or(1.);
        Self { depth, rr_tp }
    }
}

impl Trace for Path {
    #[inline(always)]
    fn trace(&self, scene: &Scene, mut sampler: Sampler, ray: R) -> Color {
        let init = (Color::ZERO, Color::ONE, ray, scene.intersect(ray), true);
        if init.3.is_none() { return scene.lenv(&ray) }

        match (0..self.depth[1]).try_fold(init,
        |(mut li, mut tp, ray, its, spec), depth| its.map(|its| {
            let (ld, res) = Direct::li(scene, &mut sampler, &its, &ray, false);
            li += tp * (ld + if spec { its.le(ray) } else { Color::ZERO });

            let (ray, its, spec) = match res {
                None => return Either::L(li),
                Some((lb, ray, its, spec))
                    => { tp *= lb; (ray, Some(its), spec) },
            };

            if depth > self.depth[0] {
                let q = F::min(tp.reduce(F::max), self.rr_tp);
                if sampler.rng() < q { return Either::L(li) }
                tp /= q;
            };

            Either::R((li, tp, ray, its, spec))
        }).unwrap_or_else(|| Either::L(li))) {
            Either::L(li) | Either::R((li, _, _, _, _)) => li,
        }
    }
}
