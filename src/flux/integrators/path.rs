use crate::flux::{ray::Ray, Scene};

use super::Integrator;

pub struct PathTracingIntegrator {
    max_depth: u32,
}

impl PathTracingIntegrator {
    pub fn new(max_depth: u32) -> Self {
        Self { max_depth }
    }

    fn background(&self, scene: &Scene, ray: &Ray) -> glam::Vec3 {
        let oc = ray.direction.normalize();

        // let a = 0.5 * (unit_direction.y + 1.0);
        // (1.0 - a) * glam::Vec3::ONE + a * glam::vec3(0.5, 0.7, 1.0)

        let theta = (-oc.y).acos();
        let phi = (-oc.z).atan2(oc.x) + std::f32::consts::PI;
        let uv = glam::vec2(
            phi / (2.0 * std::f32::consts::PI),
            theta / std::f32::consts::PI,
        );

        scene.background.evaluate(uv)
    }

    fn li_internal(
        &self,
        scene: &Scene,
        ray: &Ray,
        rng: &mut rand::rngs::StdRng,
        depth: u32,
    ) -> glam::Vec3 {
        if depth == self.max_depth {
            glam::Vec3::ZERO
        } else {
            match scene.aggregate.intersect(ray) {
                Some(int) => {
                    let le = int.primitive.material.le(&int).unwrap_or(glam::Vec3::ZERO);

                    match int.primitive.material.scatter(ray, &int, rng) {
                        Some(srec) => {
                            let scattered = int.spawn_ray(srec.direction);
                            le + srec.attenuation
                                * self.li_internal(scene, &scattered, rng, depth + 1)
                        }
                        None => le,
                    }
                }
                None => self.background(scene, ray),
            }
        }
    }
}

impl Integrator for PathTracingIntegrator {
    fn li(&self, scene: &Scene, ray: &Ray, rng: &mut rand::rngs::StdRng) -> glam::Vec3 {
        self.li_internal(scene, ray, rng, 0)
    }
}
