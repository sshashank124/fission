use crate::types::*;


#[inline(always)]
pub fn quad(a: F, b: F, c: F) -> Option<F2> {
    let dis = b * b - 4. * a * c;
    if dis < 0. {
        return None;
    }
    let disqrt = dis.sqrt();
    let q = -0.5 * (b + b.signum() * disqrt);
    let t1 = q / a;
    let t2 = c / q;
    if t1 <= t2 {
        Some(P2(t1, t2))
    } else {
        Some(P2(t2, t1))
    }
}
