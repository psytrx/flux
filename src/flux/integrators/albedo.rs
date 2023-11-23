use crate::flux::{ray::Ray, Scene};

use super::Integrator;

pub struct AlbedoIntegrator;

impl AlbedoIntegrator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Integrator for AlbedoIntegrator {
    fn li(&self, scene: &Scene, ray: &Ray, rng: &mut rand::rngs::StdRng) -> glam::Vec3 {
        match scene.aggregate.intersect(ray) {
            Some(int) => {
                // TODO: only return the first specular or diffuse hit
                match int.primitive.material.scatter(ray, &int, rng) {
                    Some(srec) => srec.attenuation,
                    None => glam::Vec3::ZERO,
                }
            }
            None => glam::Vec3::ZERO,
        }
    }
}
