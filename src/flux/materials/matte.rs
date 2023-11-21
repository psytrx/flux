use crate::flux::{interaction::Interaction, random::random_unit_vector, ray::Ray};

use super::{is_near_zero, Material, ScatterRec};

pub struct MatteMaterial {
    albedo: glam::Vec3,
}

impl MatteMaterial {
    pub fn new(albedo: glam::Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for MatteMaterial {
    fn scatter(
        &self,
        _ray_in: &Ray,
        int: &Interaction,
        rng: &mut rand::rngs::StdRng,
    ) -> Option<ScatterRec> {
        let attenuation = self.albedo;

        let direction = int.normal + random_unit_vector(rng);
        let direction = if is_near_zero(&direction) {
            int.normal
        } else {
            direction
        };

        Some(ScatterRec {
            attenuation,
            direction,
        })
    }
}
