use super::*;


pub struct Path {
    depth: I2,
    rr_throughput: F,
}

impl Path {
    #[inline(always)]
    pub fn new(depth: Option<I2>, rr_throughput: Option<F>) -> Self {
        let depth = depth.unwrap_or(A2(5, 10));
        let rr_throughput = rr_throughput.unwrap_or(0.95);
        Self { depth, rr_throughput }
    }
}

impl Trace for Path {
    #[inline(always)]
    fn trace(&self, scene: &Scene, sampler: &mut Sampler, ray: R) -> Color {
        let init = (Color::ZERO, Color::ONE, ray, scene.intersect(ray), true);
        match (0..self.depth[1]).try_fold(init,
        |(li, tp, ray, its, spec), depth| its.map(|its| {
            let (ld, res) = Direct::li(scene, sampler, &its, &ray);
            let li = li + tp * (ld + if spec { its.le(ray) }
                                     else { Color::ZERO });
            res.map(|(lb, ray, its, spec)| {
                let tp = tp * lb / if depth > self.depth[0] {
                    let q = F::min(tp.reduce(F::max), self.rr_throughput);
                    if sampler.next_1d() >= q
                    { return Either::L((li, tp, ray, Some(its), spec)) }
                    else { q }
                } else { 1. };
                Either::R((li, tp, ray, Some(its), spec))
            }).unwrap_or_else(|| Either::L((li, tp, ray, None, spec)))
        }).unwrap_or_else(|| Either::L((li, tp, ray, None, spec)))) {
            Either::L((li, _, _, _, _)) | Either::R((li, _, _, _, _)) => li,
        }
    }
}
