use super::Shape;

pub struct TriangleMesh {
    vertices: Vec<glam::Vec3>,
    indices: Vec<usize>,
}

impl TriangleMesh {
    pub fn new(vertices: Vec<glam::Vec3>, indices: Vec<usize>) -> Self {
        Self { vertices, indices }
    }

    pub fn load_obj<P: AsRef<std::path::Path> + std::fmt::Debug>(
        filename: P,
    ) -> anyhow::Result<Vec<TriangleMesh>> {
        let (models, maybe_materials) = tobj::load_obj(
            &filename,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: false,
                ..Default::default()
            },
        )?;

        let _materials = maybe_materials.unwrap_or_else(|err| {
            log::warn!(
                "Failed to load materials from obj file {:?}: {}",
                &filename,
                err
            );
            vec![]
        });

        Ok(models
            .iter()
            .enumerate()
            .map(|(i, model)| {
                let vertices: Vec<glam::Vec3> = model
                    .mesh
                    .positions
                    .chunks(3)
                    .map(|chunk| match chunk {
                        [x, y, z] => glam::vec3(*x, *y, *z),
                        _ => panic!("Invalid chunk size in obj mesh"),
                    })
                    .collect();

                let indices = model
                    .mesh
                    .indices
                    .iter()
                    .map(|index| *index as usize)
                    .collect();

                let _normals = model
                    .mesh
                    .normals
                    .chunks(3)
                    .map(|chunk| match chunk {
                        [x, y, z] => glam::vec3(*x, *y, *z),
                        _ => panic!("Invalid chunk size in obj mesh"),
                    })
                    .collect::<Vec<_>>();

                log::trace!(
                    "Loaded model {} from {:?} ({} vertices)",
                    i + 1,
                    &filename,
                    vertices.len()
                );

                Self::new(vertices, indices)
            })
            .collect())
    }
}

impl Shape for TriangleMesh {
    fn build_geometry(&self, device: embree4_sys::RTCDevice) -> embree4_sys::RTCGeometry {
        unsafe {
            let geometry =
                embree4_sys::rtcNewGeometry(device, embree4_sys::RTCGeometryType::TRIANGLE);

            let vertex_buf_ptr = embree4_sys::rtcSetNewGeometryBuffer(
                geometry,
                embree4_sys::RTCBufferType::VERTEX,
                0,
                embree4_sys::RTCFormat::FLOAT3,
                3 * std::mem::size_of::<f32>(),
                self.vertices.len(),
            );
            let vertex_buf =
                std::slice::from_raw_parts_mut(vertex_buf_ptr as *mut f32, 3 * self.vertices.len());
            for (i, vertex) in self.vertices.iter().enumerate() {
                vertex_buf[i * 3] = vertex.x;
                vertex_buf[i * 3 + 1] = vertex.y;
                vertex_buf[i * 3 + 2] = vertex.z;
            }

            let index_buf_ptr = embree4_sys::rtcSetNewGeometryBuffer(
                geometry,
                embree4_sys::RTCBufferType::INDEX,
                0,
                embree4_sys::RTCFormat::UINT3,
                3 * std::mem::size_of::<u32>(),
                self.indices.len(),
            );
            let index_buf =
                std::slice::from_raw_parts_mut(index_buf_ptr as *mut u32, self.indices.len());
            for (i, index) in self.indices.iter().enumerate() {
                index_buf[i] = *index as u32;
            }

            geometry
        }
    }

    fn uv(&self, _p: glam::Vec3) -> glam::Vec2 {
        // TODO: implement UV coordinates for meshes
        glam::Vec2::ZERO
    }
}
