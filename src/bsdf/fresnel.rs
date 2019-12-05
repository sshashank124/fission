use std::ops::{Div, Sub};

use crate::core::*;


#[inline(always)] pub fn fresnel(mut cti: F, mut ior: F2) -> F {
    if ior.reduce(F::approx_eq) { return 0. }
    if cti < 0. { ior = ior.rev(); cti = -cti; };
    let eta = ior.reduce(Div::div);
    let stt2 = eta.sq() * (1. - cti.sq());
    if stt2 > 1. { return 1. }
    let ct = A2(cti, F::sqrt(1. - stt2));
    let iors = A2(ior, ior.rev());
    let d = iors.map(|i| i.dot(ct));
    let r = iors.map(|i| (i * ct).reduce(Sub::sub)) / d;
    r.map(F::sq).mean()
}
