use super::Shape;

pub struct Quad {
    vertices: [glam::Vec3; 4],
}

impl Quad {
    pub fn new(vertices: [glam::Vec3; 4]) -> Self {
        Self { vertices }
    }
}

impl Shape for Quad {
    fn build_geometry(&self, device: embree4_sys::RTCDevice) -> embree4_sys::RTCGeometry {
        unsafe {
            let geometry = embree4_sys::rtcNewGeometry(device, embree4_sys::RTCGeometryType::QUAD);

            let vertex_buf_ptr = embree4_sys::rtcSetNewGeometryBuffer(
                geometry,
                embree4_sys::RTCBufferType::VERTEX,
                0,
                embree4_sys::RTCFormat::FLOAT3,
                3 * std::mem::size_of::<f32>(),
                4,
            );
            let vertex_buf = std::slice::from_raw_parts_mut(vertex_buf_ptr as *mut f32, 3 * 4);
            for (i, vertex) in self.vertices.iter().enumerate() {
                vertex_buf[i * 3] = vertex.x;
                vertex_buf[i * 3 + 1] = vertex.y;
                vertex_buf[i * 3 + 2] = vertex.z;
            }

            let index_buf_ptr = embree4_sys::rtcSetNewGeometryBuffer(
                geometry,
                embree4_sys::RTCBufferType::INDEX,
                0,
                embree4_sys::RTCFormat::UINT4,
                4 * std::mem::size_of::<u32>(),
                1,
            );
            let index_buf = std::slice::from_raw_parts_mut(index_buf_ptr as *mut u32, 4);
            index_buf.copy_from_slice(&[0, 1, 2, 3]);

            geometry
        }
    }

    fn uv(&self, p: glam::Vec3) -> glam::Vec2 {
        let u_vec = self.vertices[1] - self.vertices[0];
        let v_vec = self.vertices[3] - self.vertices[0];

        let pc = p - self.vertices[0];
        let u = pc.dot(u_vec) / u_vec.length_squared();
        let v = pc.dot(v_vec) / v_vec.length_squared();

        glam::vec2(u, v)
    }
}
