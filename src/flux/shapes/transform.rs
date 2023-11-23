use crate::flux::interaction::Interaction;

use super::Shape;

pub struct Transform {
    transform: glam::Affine3A,
    shape: Box<dyn Shape>,
}

impl Transform {
    pub fn new(transform: glam::Affine3A, shape: Box<dyn Shape>) -> Self {
        Self { transform, shape }
    }
}

impl Shape for Transform {
    fn build_geometry(&self, device: embree4_sys::RTCDevice) -> embree4_sys::RTCGeometry {
        unsafe {
            let shape_geom = self.shape.build_geometry(device);
            embree4_sys::rtcCommitGeometry(shape_geom);

            let sub_scene = embree4_sys::rtcNewScene(device);
            embree4_sys::rtcAttachGeometry(sub_scene, shape_geom);
            embree4_sys::rtcReleaseGeometry(shape_geom);
            embree4_sys::rtcCommitScene(sub_scene);

            let instance =
                embree4_sys::rtcNewGeometry(device, embree4_sys::RTCGeometryType::INSTANCE);
            embree4_sys::rtcSetGeometryInstancedScene(instance, sub_scene);

            let xfm = self.transform.to_cols_array();
            let xfm_ptr = xfm.as_ptr();
            embree4_sys::rtcSetGeometryTransform(
                instance,
                0,
                embree4_sys::RTCFormat::FLOAT3X4_COLUMN_MAJOR,
                xfm_ptr as _,
            );

            instance
        }
    }

    fn uv(&self, _p: glam::Vec3) -> glam::Vec2 {
        // TODO: implement UV coordinates for Transform geometry
        glam::Vec2::ZERO
    }

    fn adjust_interaction(&self, int: &mut Interaction) {
        self.shape.adjust_interaction(int);
        // we need to normalize the normal, because the transform could have scaled it
        int.normal = self.transform.transform_vector3(int.normal).normalize();
    }
}
