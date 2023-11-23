mod floor;
mod mesh;
mod quad;
mod sphere;
mod transform;

pub use floor::*;
pub use mesh::*;
pub use quad::*;
pub use sphere::*;
pub use transform::*;

use super::interaction::Interaction;

pub trait Shape {
    fn build_geometry(&self, device: embree4_sys::RTCDevice) -> embree4_sys::RTCGeometry;
    fn uv(&self, p: glam::Vec3) -> glam::Vec2;

    fn adjust_interaction(&self, _int: &mut Interaction) {}
}
