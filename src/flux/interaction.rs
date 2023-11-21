use super::{materials::Material, ray::Ray};

pub struct Interaction {
    pub point: glam::Vec3,
    pub normal: glam::Vec3,
    pub front_face: bool,
    pub material: std::rc::Rc<dyn Material>,
}

impl Interaction {
    pub fn spawn_ray(&self, direction: glam::Vec3) -> Ray {
        Ray::new(self.point, direction)
    }
}
