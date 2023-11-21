use super::{CameraSample, Sampler};

pub struct UniformRandomSampler {
    pub samples_per_pixel: usize,
}

impl UniformRandomSampler {
    pub fn new(samples_per_pixel: usize) -> Self {
        Self { samples_per_pixel }
    }
}

impl Sampler for UniformRandomSampler {
    fn camera_samples(
        &self,
        p_raster: glam::Vec2,
        rng: &mut rand::rngs::StdRng,
    ) -> Vec<CameraSample> {
        (0..self.samples_per_pixel)
            .map(|_| {
                let p_film = p_raster + rand::Rng::gen::<glam::Vec2>(rng);
                let p_lens = rand::Rng::gen::<glam::Vec2>(rng);
                CameraSample { p_film, p_lens }
            })
            .collect()
    }
}
