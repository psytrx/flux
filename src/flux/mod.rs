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
use ray::*;

use self::{
    cameras::DummyCamera,
    materials::MatteMaterial,
    primitive::Primitive,
    samplers::{Sampler, UniformRandomSampler},
    scene::Scene,
    shapes::{Floor, Sphere},
};

pub fn render_film(resolution: glam::UVec2, max_depth: u32) -> Film {
    let scene = {
        let primitives = {
            let material = std::rc::Rc::new(MatteMaterial::new(glam::Vec3::splat(0.5)));

            let floor = {
                let shape = Box::new(Floor::new());
                Primitive::new(shape, material.clone())
            };
            let sphere = {
                let shape = Box::new(Sphere::new(glam::vec3(0.0, 1.0, 0.0), 1.0));
                Primitive::new(shape, material.clone())
            };

            vec![floor, sphere]
        };

        let camera = Box::new(DummyCamera::new(resolution));

        Scene::new(primitives, camera)
    };

    let sampler = UniformRandomSampler::new(32);
    let mut rng = <rand::rngs::StdRng as rand::SeedableRng>::seed_from_u64(0);

    let mut film = Film::new(resolution);
    for y in 0..resolution.y {
        for x in 0..resolution.y {
            let p_raster = glam::vec2(x as f32, y as f32);
            let cam_samples = sampler.camera_samples(p_raster, &mut rng);

            for sample in cam_samples {
                let ray = scene.camera.ray(sample.p_film);
                let li = ray_color(&scene, &ray, &mut rng, max_depth);
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
