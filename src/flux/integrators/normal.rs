use crate::flux::{ray::Ray, Scene};

use super::Integrator;

pub struct NormalIntegrator;

impl NormalIntegrator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Integrator for NormalIntegrator {
    fn li(&self, scene: &Scene, ray: &Ray, _rng: &mut rand::rngs::StdRng) -> glam::Vec3 {
        match scene.aggregate.intersect(ray) {
            Some(int) => 0.5 * (int.normal + glam::Vec3::ONE),
            None => glam::Vec3::ZERO,
        }
    }
}
