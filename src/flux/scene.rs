use super::{accel::EmbreeAccel, cameras::Camera, textures::Texture};

pub struct Scene {
    pub aggregate: EmbreeAccel,
    pub camera: Box<dyn Camera>,
    pub background: Box<dyn Texture>,
}

impl Scene {
    pub fn new(
        primitives: Vec<super::primitive::Primitive>,
        camera: Box<dyn Camera>,
        background: Box<dyn Texture>,
    ) -> Self {
        let aggregate = EmbreeAccel::build(primitives);
        Self {
            aggregate,
            camera,
            background,
        }
    }
}
