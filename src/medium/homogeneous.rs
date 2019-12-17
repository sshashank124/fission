use super::*;


#[derive(Debug, PartialEq)]
pub struct Homogeneous {
    ss: Color,
    st: Color,
    phase: PhaseFn,
}

impl Homogeneous {
    pub fn new(sa: Color, ss: Color, g: F) -> Self
    { Self { ss, st: sa + ss, phase: HenyeyGreenstein::new(g) } }

    #[inline(always)] fn tr(&self, d: F) -> Color
    { Color((-self.st * F::min(d, F::MAX_VAL)).map(F::exp)) }
}

impl MediumT for Homogeneous {
    #[inline(always)] fn tr(&self, ray: &R) -> Color { self.tr(ray.t) }

    #[inline(always)]
    fn sample(&self, ray: &R, sampler: &mut Sampler) -> (Color, Option<Its>) {
        let chan = F::discrete(sampler.next_1d(), 3);
        let t = F::min(ray.t, -F::ln(1. - sampler.next_1d()) / self.st[chan]);
        let in_medium = t < ray.t;
        let its = if !in_medium { None }
                  else { Some(Its::new(ray.at(t), N::v(-ray.d), F2::ZERO, t)
                                  .with_phase(&self.phase)) };
        let tr = self.tr(t);
        let pdf = (tr * if in_medium { self.st } else { Color::ONE }).mean();
        let pdf = if pdf <= 0. { 1. } else { pdf.inv() };
        (tr * if in_medium { self.ss } else { Color::ONE } * pdf, its)
    }
}
