pub mod accel;
pub mod cameras;
mod film;
mod interaction;
mod primitive;
mod ray;
pub mod shapes;

pub use film::*;
use ray::*;

use self::{
    accel::EmbreeAccel,
    cameras::{Camera, DummyCamera},
    primitive::Primitive,
    shapes::{Floor, Sphere},
};

pub fn render_film(resolution: glam::UVec2) -> Film {
    let primitives = vec![
        Primitive::new(Box::new(Floor::new())),
        Primitive::new(Box::new(Sphere::new(glam::vec3(0.0, 1.0, 0.0), 1.0))),
    ];
    let accel = EmbreeAccel::build(&primitives);

    let camera = DummyCamera::new(resolution);

    Film::from_fn(resolution, |x, y| {
        let p_raster = glam::vec2(x as f32, y as f32);
        let ray = camera.ray(p_raster);
        ray_color(&accel, &ray)
    })
}

fn ray_color(accel: &EmbreeAccel, ray: &Ray) -> glam::Vec3 {
    match accel.intersect(ray) {
        Some(int) => 0.5 * (int.normal + glam::Vec3::ONE),
        None => background(ray),
    }
}

fn background(ray: &Ray) -> glam::Vec3 {
    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * glam::Vec3::ONE + a * glam::vec3(0.5, 0.7, 1.0)
}
