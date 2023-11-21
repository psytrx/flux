pub struct Ray {
    pub origin: glam::Vec3,
    pub direction: glam::Vec3,
}

impl Ray {
    pub fn new(origin: glam::Vec3, direction: glam::Vec3) -> Self {
        Self { origin, direction }
    }
}

impl From<&Ray> for embree4_sys::RTCRay {
    fn from(value: &Ray) -> Self {
        embree4_sys::RTCRay {
            org_x: value.origin.x,
            org_y: value.origin.y,
            org_z: value.origin.z,
            dir_x: value.direction.x,
            dir_y: value.direction.y,
            dir_z: value.direction.z,
            tnear: 0.0001,
            tfar: std::f32::INFINITY,
            ..Default::default()
        }
    }
}
