use crate::flux::{
    materials::Material,
    shapes::{Floor, Sphere},
    Primitive,
};

pub fn floor(mat: std::rc::Rc<dyn Material>) -> Primitive {
    let shape = Box::new(Floor::new());
    Primitive::new(shape, mat)
}

pub fn sphere(x: f32, y: f32, z: f32, r: f32, mat: std::rc::Rc<dyn Material>) -> Primitive {
    let shape = Box::new(Sphere::new(glam::vec3(x, y, z), r));
    Primitive::new(shape, mat)
}
