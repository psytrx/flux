use super::Shape;

pub struct Floor;

impl Floor {
    pub fn new() -> Self {
        Floor
    }
}

impl Shape for Floor {
    fn build_geometry(&self, device: embree4_sys::RTCDevice) -> embree4_sys::RTCGeometry {
        unsafe {
            let geometry = embree4_sys::rtcNewGeometry(device, embree4_sys::RTCGeometryType::USER);

            embree4_sys::rtcSetGeometryUserPrimitiveCount(geometry, 1);
            embree4_sys::rtcSetGeometryBoundsFunction(
                geometry,
                Some(bounds_fn),
                std::ptr::null_mut(),
            );
            embree4_sys::rtcSetGeometryIntersectFunction(geometry, Some(intersect_fn));

            geometry
        }
    }
}

unsafe extern "C" fn bounds_fn(args: *const embree4_sys::RTCBoundsFunctionArguments) {
    let args = *args;
    *args.bounds_o = embree4_sys::RTCBounds {
        lower_x: -999_999.0,
        lower_y: -0.0001,
        lower_z: -999_999.0,
        align0: Default::default(),
        upper_x: 999_999.0,
        upper_y: 0.0001,
        upper_z: 999_999.0,
        align1: Default::default(),
    }
}

unsafe extern "C" fn intersect_fn(args: *const embree4_sys::RTCIntersectFunctionNArguments) {
    let args = *args;
    debug_assert_eq!(1, args.N);

    let valid = *args.valid;
    if valid == 0 {
        return;
    }

    let ray_hit_ptr = args.rayhit as *mut embree4_sys::RTCRayHit;
    let ray_hit = &mut *ray_hit_ptr;

    if ray_hit.ray.dir_y == 0.0 {
        return;
    }

    let t = -ray_hit.ray.org_y / ray_hit.ray.dir_y;
    if t < ray_hit.ray.tnear || t > ray_hit.ray.tfar {
        return;
    }

    ray_hit.hit.Ng_x = 0.0;
    ray_hit.hit.Ng_z = 0.0;
    if ray_hit.ray.org_y > 0.0 {
        ray_hit.hit.Ng_y = 1.0;
    } else {
        ray_hit.hit.Ng_y = -1.0;
    }

    ray_hit.ray.tfar = t;
    ray_hit.hit.primID = args.primID;
    ray_hit.hit.geomID = args.geomID;
}
