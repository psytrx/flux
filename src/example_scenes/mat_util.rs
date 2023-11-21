use crate::flux::materials::{Material, MatteMaterial};

pub fn matte(r: f32, g: f32, b: f32) -> std::rc::Rc<dyn Material> {
    std::rc::Rc::new(MatteMaterial::new(glam::vec3(r, g, b)))
}

pub fn metal(r: f32, g: f32, b: f32, fuzz: f32) -> std::rc::Rc<dyn Material> {
    std::rc::Rc::new(crate::flux::materials::MetalMaterial::new(
        glam::vec3(r, g, b),
        fuzz,
    ))
}

pub fn dielectric(r: f32, g: f32, b: f32, ior: f32) -> std::rc::Rc<dyn Material> {
    std::rc::Rc::new(crate::flux::materials::DielectricMaterial::new(
        glam::vec3(r, g, b),
        ior,
    ))
}
