use crate::flux::primitive::Primitive;

pub struct EmbreeAccel {
    pub scene: embree4_sys::RTCScene,
}

impl EmbreeAccel {
    pub fn build(primitives: &[Primitive]) -> EmbreeAccel {
        let scene = unsafe {
            let device = embree4_sys::rtcNewDevice(b"verbose=0" as *const _ as _);

            let scene = embree4_sys::rtcNewScene(device);
            embree4_sys::rtcSetSceneBuildQuality(scene, embree4_sys::RTCBuildQuality::HIGH);
            embree4_sys::rtcSetSceneFlags(scene, embree4_sys::RTCSceneFlags::ROBUST);

            for (id, prim) in primitives.iter().enumerate() {
                let geometry_id = id as u32;
                let geometry = prim.build_geometry(device);

                embree4_sys::rtcCommitGeometry(geometry);
                embree4_sys::rtcAttachGeometryByID(scene, geometry, geometry_id);
                embree4_sys::rtcReleaseGeometry(geometry);
            }

            embree4_sys::rtcCommitScene(scene);

            scene
        };

        Self { scene }
    }
}
