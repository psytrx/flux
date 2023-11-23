use super::{ray::Ray, Primitive};

pub struct Interaction<'a> {
    pub point: glam::Vec3,
    pub normal: glam::Vec3,
    pub front_face: bool,
    pub uv: glam::Vec2,
    pub primitive: &'a Primitive,
}

impl<'a> Interaction<'a> {
    pub fn spawn_ray(&self, direction: glam::Vec3) -> Ray {
        Ray::new(self.point, direction)
    }
}
