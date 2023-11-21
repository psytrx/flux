mod film;
mod ray;

pub use film::*;
use ray::*;

pub fn render_film(resolution: glam::UVec2) -> Film {
    let upper_left = glam::vec3(-2.0, 2.0, 0.0);
    let horizontal = glam::vec3(4.0, 0.0, 0.0);
    let vertical = glam::vec3(0.0, -4.0, 0.0);
    let origin = glam::vec3(0.0, 0.0, -4.0);

    Film::from_fn(resolution, |x, y| {
        let u = x as f32 / resolution.x as f32;
        let v = y as f32 / resolution.y as f32;

        let target = upper_left + u * horizontal + v * vertical;
        let ray = Ray::new(origin, target - origin);

        ray_color(ray)
    })
}

fn ray_color(ray: Ray) -> glam::Vec3 {
    match hit_sphere(&ray) {
        Some(interaction) => 0.5 * (interaction.n + glam::Vec3::ONE),
        None => background(ray),
    }
}

fn background(ray: Ray) -> glam::Vec3 {
    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * glam::Vec3::ONE + a * glam::vec3(0.5, 0.7, 1.0)
}

fn hit_sphere(ray: &Ray) -> Option<Interaction> {
    let center = glam::Vec3::ZERO;
    let radius = 1.0;

    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        None
    } else {
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        let p = ray.origin + t * ray.direction;
        let n = (p - center).normalize();
        Some(Interaction { _p: p, n })
    }
}

struct Interaction {
    _p: glam::Vec3,
    n: glam::Vec3,
}
