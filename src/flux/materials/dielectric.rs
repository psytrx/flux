use crate::flux::{interaction::Interaction, ray::Ray};

use super::{reflect, refract, Material, ScatterRec};

pub struct DielectricMaterial {
    albedo: glam::Vec3,
    ior: f32,
}

impl DielectricMaterial {
    pub fn new(albedo: glam::Vec3, ior: f32) -> Self {
        Self { albedo, ior }
    }
}

impl Material for DielectricMaterial {
    fn scatter(
        &self,
        ray_in: &Ray,
        int: &Interaction,
        rng: &mut rand::rngs::StdRng,
    ) -> Option<ScatterRec> {
        let attenuation = self.albedo;

        let refraction_ratio = if int.front_face {
            1.0 / self.ior
        } else {
            self.ior
        };

        let unit_direction = ray_in.direction.normalize();
        let cos_theta = (-unit_direction).dot(int.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || schlick(cos_theta, refraction_ratio) > rand::Rng::gen(rng) {
                reflect(unit_direction, int.normal)
            } else {
                refract(unit_direction, int.normal, refraction_ratio)
            };

        Some(ScatterRec {
            attenuation,
            direction,
        })
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
