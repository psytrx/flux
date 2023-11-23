mod constant;
mod image;

pub use constant::*;
pub use image::*;

pub trait Texture {
    fn evaluate(&self, uv: glam::Vec2) -> glam::Vec3;
}
