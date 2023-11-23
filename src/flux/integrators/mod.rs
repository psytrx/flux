mod path;

pub use path::*;

use super::{ray::Ray, Scene};

pub trait Integrator {
    fn li(&self, scene: &Scene, ray: &Ray, rng: &mut rand::rngs::StdRng) -> glam::Vec3;
}
