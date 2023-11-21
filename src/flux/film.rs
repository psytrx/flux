pub struct Film {
    resolution: glam::UVec2,
    pixels: Vec<Pixel>,
}

impl Film {
    pub fn new(resolution: glam::UVec2) -> Self {
        let dim = resolution.x * resolution.y;
        let pixels = vec![Pixel::ZERO; dim as usize];
        Self { resolution, pixels }
    }

    pub fn from_fn(resolution: glam::UVec2, f: impl Fn(u32, u32) -> glam::Vec3) -> Self {
        let dim = resolution.x * resolution.y;
        let mut pixels = vec![Pixel::ZERO; dim as usize];
        for y in 0..resolution.y {
            for x in 0..resolution.x {
                let index = y * resolution.x + x;
                let pixel = &mut pixels[index as usize];
                pixel.spectrum_sum = f(x, y);
                pixel.weight_sum = 1.0;
            }
        }
        Self::from_pixels(resolution, pixels)
    }

    fn from_pixels(resolution: glam::UVec2, pixels: Vec<Pixel>) -> Self {
        Self { resolution, pixels }
    }
}

impl From<Film> for image::Rgb32FImage {
    fn from(val: Film) -> Self {
        image::Rgb32FImage::from_fn(val.resolution.x, val.resolution.y, |x, y| {
            let index = y * val.resolution.x + x;
            let pixel = &val.pixels[index as usize];
            let color = pixel.spectrum_sum / pixel.weight_sum;
            image::Rgb([color.x, color.y, color.z])
        })
    }
}

#[derive(Clone, Copy)]
struct Pixel {
    spectrum_sum: glam::Vec3,
    weight_sum: f32,
}

impl Pixel {
    const ZERO: Self = Self {
        spectrum_sum: glam::Vec3::ZERO,
        weight_sum: 0.0,
    };
}
