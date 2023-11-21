use super::shapes::Shape;

pub struct Primitive {
    pub shape: Box<dyn Shape>,
}

impl Primitive {
    pub fn new(shape: Box<dyn Shape>) -> Self {
        Self { shape }
    }

    pub unsafe fn build_geometry(
        &self,
        device: embree4_sys::RTCDevice,
    ) -> embree4_sys::RTCGeometry {
        self.shape.build_geometry(device)
    }
}
