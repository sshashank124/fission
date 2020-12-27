use super::*;

#[derive(Debug, Deserialize)]
pub struct Perspective {
    #[serde(rename="fov", deserialize_with="de_fov_scale")]
    fov_scale: F,
    #[serde(default, rename="lens_radius")]
    lens_r:    F,
    #[serde(default, rename="focal_distance")]
    fd:        F,
}

impl Perspective {
    #[inline(always)]
    pub fn ray_at(&self, point: F2, sampler: &mut Sampler) -> R {
        let d = V::from(F3::a2a(point * self.fov_scale, 1.));
        let ray = R::unbounded(P::ZERO, d);
        if F::approx_zero(self.lens_r) { ray } else {
            let focus_point = ray.at(self.fd / ray.d[Z]);
            let o = P::from(F3::a2a(UniformDisk::warp(sampler.next_2d())
                                    * self.lens_r, 0.));
            R::unbounded(o, focus_point - o)
        }
    }
}

fn de_fov_scale<'de, D>(de: D) -> Result<F, D::Error>
where D: serde::Deserializer<'de>
{ F::deserialize(de).map(|fov| F::tand(0.5 * fov)) }
