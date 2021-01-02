use std::ops::Div;

use super::*;

#[inline(always)] pub fn eta(ior: F2) -> F { ior.reduce(Div::div) }

// (fresnel coefficient, cos theta out, eta)
#[inline(always)] pub fn fresnel(ct_i: F, eta: F) -> (F, F, F) {
    let (s, f) = if ct_i > 0. { (eta, -1.) } else { (eta.inv(), 1.) };
    let ct_t2 = 1. - s.sq() * (1. - ct_i.sq());
    if ct_t2 <= 0. { return (1., 0., 1.) }
    let ct_i = F::abs(ct_i);
    let ct_t = F::sqrt(ct_t2);
    let r = A2(-s.mul_add(ct_t, ct_i) / s.mul_add(ct_t, ct_i),
               s.mul_add(ct_i, - ct_t) / s.mul_add(ct_i, ct_t));
    (r.map(F::sq).mean(), ct_t * f, s)
}
