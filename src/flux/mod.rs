pub mod accel;
pub mod cameras;
mod denoise;
mod film;
pub mod integrators;
mod interaction;
pub mod materials;
mod primitive;
mod random;
mod ray;
pub mod samplers;
mod scene;
pub mod shapes;
pub mod textures;

pub use denoise::*;
pub use film::*;
pub use primitive::*;
use samplers::Sampler;
pub use scene::*;

use self::integrators::Integrator;

pub fn render_film(scene: &Scene, sampler: impl Sampler, integrator: impl Integrator) -> Film {
    let mut rng = <rand::rngs::StdRng as rand::SeedableRng>::seed_from_u64(0);

    let resolution = scene.camera.resoltuion();

    let mut film = Film::new(resolution);
    for y in 0..resolution.y {
        for x in 0..resolution.x {
            let p_raster = glam::vec2(x as f32, y as f32);
            let cam_samples = sampler.camera_samples(p_raster, &mut rng);

            for sample in cam_samples {
                let ray = scene.camera.ray(&sample);
                let li = integrator.li(scene, &ray, &mut rng);
                film.add_sample(p_raster, li);
            }
        }
    }

    film
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
