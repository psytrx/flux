mod matte;

pub use matte::*;

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
