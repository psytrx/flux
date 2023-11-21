mod dielectric;
mod matte;
mod metal;

pub use dielectric::*;
pub use matte::*;
pub use metal::*;

use super::{interaction::Interaction, ray::Ray};

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        int: &Interaction,
        rng: &mut rand::rngs::StdRng,
    ) -> Option<ScatterRec>;
}

pub struct ScatterRec {
    pub attenuation: glam::Vec3,
    pub direction: glam::Vec3,
}

fn is_near_zero(v: &glam::Vec3) -> bool {
    let s = 1e-4;
    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}

fn reflect(v: glam::Vec3, n: glam::Vec3) -> glam::Vec3 {
    v - 2.0 * v.dot(n) * n
}

fn refract(uv: glam::Vec3, n: glam::Vec3, etai_over_etat: f32) -> glam::Vec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}
