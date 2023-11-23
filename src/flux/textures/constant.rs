use super::Texture;

pub struct ConstantTexture {
    color: glam::Vec3,
}

impl ConstantTexture {
    pub fn new(color: glam::Vec3) -> Self {
        Self { color }
    }
}

impl Texture for ConstantTexture {
    fn evaluate(&self, _uv: glam::Vec2) -> glam::Vec3 {
        self.color
    }
}
