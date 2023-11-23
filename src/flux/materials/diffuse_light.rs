use crate::flux::textures::Texture;

use super::Material;

pub struct DiffuseLightMaterial {
    pub emit: Box<dyn Texture>,
}

impl DiffuseLightMaterial {
    pub fn new(emit: Box<dyn Texture>) -> Self {
        Self { emit }
    }
}

impl Material for DiffuseLightMaterial {
    fn scatter(
        &self,
        _ray_in: &crate::flux::ray::Ray,
        _int: &crate::flux::interaction::Interaction,
        _rng: &mut rand::rngs::StdRng,
    ) -> Option<super::ScatterRec> {
        None
    }

    fn le(&self, int: &crate::flux::interaction::Interaction) -> Option<glam::Vec3> {
        if int.front_face {
            Some(self.emit.evaluate(int.uv))
        } else {
            None
        }
    }
}
