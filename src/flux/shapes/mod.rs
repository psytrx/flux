mod floor;
mod sphere;

pub use floor::*;
pub use sphere::*;

pub trait Shape {
    fn build_geometry(&self, device: embree4_sys::RTCDevice) -> embree4_sys::RTCGeometry;
}
