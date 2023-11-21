mod film;

pub use film::*;

pub fn render_film(resolution: glam::UVec2) -> Film {
    Film::from_fn(resolution, |x, y| {
        let r = x as f32 / resolution.x as f32;
        let g = y as f32 / resolution.y as f32;
        let b = 0.0;
        glam::vec3(r, g, b)
    })
}
