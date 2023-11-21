use super::{materials::Material, shapes::Shape};

pub struct Primitive {
    pub shape: Box<dyn Shape>,
    pub material: std::rc::Rc<dyn Material>,
}

impl Primitive {
    pub fn new(shape: Box<dyn Shape>, material: std::rc::Rc<dyn Material>) -> Self {
        Self { shape, material }
    }

    pub unsafe fn build_geometry(
        &self,
        device: embree4_sys::RTCDevice,
    ) -> embree4_sys::RTCGeometry {
        self.shape.build_geometry(device)
    }
}
