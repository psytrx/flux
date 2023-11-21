use super::{accel::EmbreeAccel, cameras::Camera};

pub struct Scene {
    pub aggregate: EmbreeAccel,
    pub camera: Box<dyn Camera>,
}

impl Scene {
    pub fn new(primitives: Vec<super::primitive::Primitive>, camera: Box<dyn Camera>) -> Self {
        let aggregate = EmbreeAccel::build(primitives);
        Self { aggregate, camera }
    }
}
