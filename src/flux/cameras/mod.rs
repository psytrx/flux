use super::ray::Ray;

pub trait Camera {
    fn ray(&self, p_raster: glam::Vec2) -> Ray;
}

pub struct DummyCamera {
    resolution: glam::UVec2,
}

impl DummyCamera {
    pub fn new(resolution: glam::UVec2) -> Self {
        Self { resolution }
    }
}

impl Camera for DummyCamera {
    fn ray(&self, p_film: glam::Vec2) -> Ray {
        let uv = p_film / self.resolution.as_vec2();

        let upper_left = glam::vec3(-2.0, 2.5, 0.0);
        let horizontal = glam::vec3(4.0, 0.0, 0.0);
        let vertical = glam::vec3(0.0, -4.0, 0.0);
        let origin = glam::vec3(0.0, 1.5, -4.0);

        let target = upper_left + uv.x * horizontal + uv.y * vertical;

        Ray::new(origin, target - origin)
    }
}
