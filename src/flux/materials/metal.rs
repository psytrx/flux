use crate::flux::{interaction::Interaction, random::random_unit_vector, ray::Ray};

use super::{is_near_zero, reflect, Material, ScatterRec};

pub struct MetalMaterial {
    albedo: glam::Vec3,
    fuzz: f32,
}

impl MetalMaterial {
    pub fn new(albedo: glam::Vec3, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for MetalMaterial {
    fn scatter(
        &self,
        _ray_in: &Ray,
        int: &Interaction,
        rng: &mut rand::rngs::StdRng,
    ) -> Option<ScatterRec> {
        let attenuation = self.albedo;

        let reflected = reflect(_ray_in.direction.normalize(), int.normal);
        let direction = reflected + self.fuzz * random_unit_vector(rng);
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
