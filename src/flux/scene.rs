use super::{accel::EmbreeAccel, cameras::Camera, textures::ImageTexture};

pub struct Scene {
    pub aggregate: EmbreeAccel,
    pub camera: Box<dyn Camera>,
    pub background: ImageTexture,
}

impl Scene {
    pub fn new(
        primitives: Vec<super::primitive::Primitive>,
        camera: Box<dyn Camera>,
        background: ImageTexture,
    ) -> Self {
        let aggregate = EmbreeAccel::build(primitives);
        Self {
            aggregate,
            camera,
            background,
        }
    }
}
