use crate::flux::{interaction::Interaction, primitive::Primitive, ray::Ray};

pub struct EmbreeAccel {
    primitives: Vec<Primitive>,
    scene: embree4_sys::RTCScene,
}

impl EmbreeAccel {
    pub fn build(primitives: Vec<Primitive>) -> EmbreeAccel {
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

        Self { primitives, scene }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Interaction> {
        let mut ray_hit = embree4_sys::RTCRayHit {
            ray: ray.into(),
            hit: Default::default(),
        };

        unsafe {
            embree4_sys::rtcIntersect1(self.scene, &mut ray_hit, std::ptr::null_mut());
        }

        if ray_hit.hit.geomID == embree4_sys::RTC_INVALID_GEOMETRY_ID {
            None
        } else {
            let t = ray_hit.ray.tfar;
            let point = ray.origin + t * ray.direction;

            // "The hit contains the unnormalized geometric normal in object space [...]"
            let embree_normal =
                glam::Vec3::new(ray_hit.hit.Ng_x, ray_hit.hit.Ng_y, ray_hit.hit.Ng_z);
            let outward_normal = embree_normal.normalize();

            let front_face = ray.direction.dot(outward_normal) < 0.0;
            let normal = if front_face {
                outward_normal
            } else {
                -outward_normal
            };

            let primitive = &self.primitives[ray_hit.hit.geomID as usize];
            let uv = primitive.shape.uv(point);

            let mut int = Interaction {
                point,
                normal,
                front_face,
                uv,
                primitive,
            };

            primitive.shape.adjust_interaction(&mut int);

            Some(int)
        }
    }
}
