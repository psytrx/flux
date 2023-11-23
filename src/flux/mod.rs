pub mod accel;
pub mod cameras;
mod film;
mod interaction;
pub mod materials;
mod primitive;
mod random;
mod ray;
pub mod samplers;
mod scene;
pub mod shapes;

pub use film::*;
pub use primitive::*;
use ray::*;
pub use scene::*;

use self::samplers::{Sampler, UniformRandomSampler};

pub fn render_film(scene: &Scene, max_depth: u32) -> Film {
    let sampler = UniformRandomSampler::new(64);
    let mut rng = <rand::rngs::StdRng as rand::SeedableRng>::seed_from_u64(0);

    let resolution = scene.camera.resoltuion();

    let mut film = Film::new(resolution);
    for y in 0..resolution.y {
        for x in 0..resolution.x {
            let p_raster = glam::vec2(x as f32, y as f32);
            let cam_samples = sampler.camera_samples(p_raster, &mut rng);

            for sample in cam_samples {
                let ray = scene.camera.ray(&sample);
                let li = ray_color(scene, &ray, &mut rng, max_depth);
                film.add_sample(p_raster, li);
            }
        }
    }

    film
}

fn ray_color(scene: &Scene, ray: &Ray, rng: &mut rand::rngs::StdRng, depth: u32) -> glam::Vec3 {
    if depth == 0 {
        glam::Vec3::ZERO
    } else {
        match scene.aggregate.intersect(ray) {
            Some(int) => match int.material.scatter(ray, &int, rng) {
                Some(srec) => {
                    let scattered = int.spawn_ray(srec.direction);
                    srec.attenuation * ray_color(scene, &scattered, rng, depth - 1)
                }
                None => glam::Vec3::ZERO,
            },
            None => background(ray),
        }
    }
}

fn background(ray: &Ray) -> glam::Vec3 {
    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * glam::Vec3::ONE + a * glam::vec3(0.5, 0.7, 1.0)
}

fn uniform_sample_disk(u: glam::Vec2) -> glam::Vec2 {
    let u_offset = 2.0 * u - glam::Vec2::ONE;

    if u_offset.x == 0.0 && u_offset.y == 0.0 {
        glam::Vec2::ZERO
    } else {
        let (r, theta) = if u_offset.x.abs() > u_offset.y.abs() {
            (
                u_offset.x,
                std::f32::consts::PI / 4.0 * (u_offset.y / u_offset.x),
            )
        } else {
            (
                u_offset.y,
                std::f32::consts::PI / 2.0 - std::f32::consts::PI / 4.0 * (u_offset.x / u_offset.y),
            )
        };

        glam::vec2(r * theta.cos(), r * theta.sin())
    }
}
