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

    fn from_spectra(resolution: glam::UVec2, spectra: Vec<glam::Vec3>) -> Self {
        let pixels = spectra
            .into_iter()
            .map(|spectrum| Pixel {
                spectrum_sum: spectrum,
                weight_sum: 1.0,
            })
            .collect();
        Self { resolution, pixels }
    }

    pub fn add_sample(&mut self, p_film: glam::Vec2, spectrum: glam::Vec3) {
        let x = p_film.x as u32;
        let y = p_film.y as u32;
        let index = y * self.resolution.x + x;

        let pixel = &mut self.pixels[index as usize];

        pixel.spectrum_sum += spectrum;
        pixel.weight_sum += 1.0;
    }

    fn mapped_spectra(&self, f: impl Fn(glam::Vec3) -> glam::Vec3) -> Film {
        let pixels = self.pixels.iter().map(|pixel| f(pixel.color())).collect();
        Self::from_spectra(self.resolution, pixels)
    }

    pub fn gamma_corrected(&self, gamma: f32) -> Film {
        self.mapped_spectra(|color| {
            glam::vec3(
                color.x.powf(1.0 / gamma),
                color.y.powf(1.0 / gamma),
                color.z.powf(1.0 / gamma),
            )
        })
    }
}

impl From<Film> for image::Rgb32FImage {
    fn from(val: Film) -> Self {
        image::Rgb32FImage::from_fn(val.resolution.x, val.resolution.y, |x, y| {
            let index = y * val.resolution.x + x;
            let pixel = &val.pixels[index as usize];
            let color = pixel.color();
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

    fn color(&self) -> glam::Vec3 {
        debug_assert!(self.weight_sum != 0.0);
        self.spectrum_sum / self.weight_sum
    }
}
