use std::ops::Div;

use super::*;

#[inline(always)] pub fn eta(ior: Option<F2>) -> F
{ ior.unwrap_or(A2(1.000_277, 1.5046)).reduce(Div::div) }

// (fresnel coefficient, cos theta out, eta)
#[inline(always)] pub fn fresnel(cti: F, eta: F) -> (F, F, F) {
    let (s, f) = if cti > 0. { (eta, -1.) } else { (eta.inv(), 1.) };
    let ctt2 = 1. - s.sq() * (1. - cti.sq());
    if ctt2 <= 0. { return (1., 0., 1.) }
    let cti = F::abs(cti);
    let ctt = F::sqrt(ctt2);
    let r = A2((cti - s * ctt) / (cti + s * ctt),
               (s * cti - ctt) / (s * cti + ctt));
    (r.map(F::sq).mean(), ctt * f, s)
}
