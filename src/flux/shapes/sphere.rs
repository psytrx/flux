use super::Shape;

pub struct Sphere {
    center: glam::Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: glam::Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Shape for Sphere {
    fn build_geometry(&self, device: embree4_sys::RTCDevice) -> embree4_sys::RTCGeometry {
        unsafe {
            let geometry =
                embree4_sys::rtcNewGeometry(device, embree4_sys::RTCGeometryType::SPHERE_POINT);

            let buffer_ptr = embree4_sys::rtcSetNewGeometryBuffer(
                geometry,
                embree4_sys::RTCBufferType::VERTEX,
                0,
                embree4_sys::RTCFormat::FLOAT4,
                4 * std::mem::size_of::<f32>(),
                1,
            );

            let buffer = std::slice::from_raw_parts_mut(buffer_ptr as *mut f32, 4);
            buffer.copy_from_slice(&[self.center.x, self.center.y, self.center.z, self.radius]);

            geometry
        }
    }
}
