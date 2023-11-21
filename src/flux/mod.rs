pub mod accel;
mod film;
mod primitive;
mod ray;
pub mod shapes;

pub use film::*;
use ray::*;

use self::{
    accel::EmbreeAccel,
    primitive::Primitive,
    shapes::{Floor, Sphere},
};

pub fn render_film(resolution: glam::UVec2) -> Film {
    let upper_left = glam::vec3(-2.0, 2.5, 0.0);
    let horizontal = glam::vec3(4.0, 0.0, 0.0);
    let vertical = glam::vec3(0.0, -4.0, 0.0);
    let origin = glam::vec3(0.0, 1.5, -4.0);

    let primitives = vec![
        Primitive::new(Box::new(Floor::new())),
        Primitive::new(Box::new(Sphere::new(glam::vec3(0.0, 1.0, 0.0), 1.0))),
    ];
    let accel = EmbreeAccel::build(&primitives);

    Film::from_fn(resolution, |x, y| {
        let u = x as f32 / resolution.x as f32;
        let v = y as f32 / resolution.y as f32;

        let target = upper_left + u * horizontal + v * vertical;
        let ray = Ray::new(origin, target - origin);

        ray_color(&accel, &ray)
    })
}

fn ray_color(accel: &EmbreeAccel, ray: &Ray) -> glam::Vec3 {
    let mut ray_hit = embree4_sys::RTCRayHit {
        ray: embree4_sys::RTCRay::from(ray),
        hit: Default::default(),
    };

    unsafe {
        embree4_sys::rtcIntersect1(accel.scene, &mut ray_hit, std::ptr::null_mut());
    }

    if ray_hit.hit.geomID == embree4_sys::RTC_INVALID_GEOMETRY_ID {
        background(ray)
    } else {
        let n = glam::Vec3::new(ray_hit.hit.Ng_x, ray_hit.hit.Ng_y, ray_hit.hit.Ng_z).normalize();
        0.5 * (n + glam::Vec3::ONE)
    }
}

fn background(ray: &Ray) -> glam::Vec3 {
    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * glam::Vec3::ONE + a * glam::vec3(0.5, 0.7, 1.0)
}
