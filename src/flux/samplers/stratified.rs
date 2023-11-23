use super::{CameraSample, Sampler};

pub struct StratifiedSampler {
    nx_ny: usize,
}

impl StratifiedSampler {
    pub fn new(samples_per_pixel: usize) -> Self {
        let nx_ny = ((samples_per_pixel as f32).sqrt().floor() as usize).max(1);

        if nx_ny * nx_ny != samples_per_pixel {
            let new_samples_per_pixel = nx_ny * nx_ny;
            log::warn!("StratifiedSampler exptects samples_per_pixel to be a perfect square number, but got {}. Falling back to {}",
                samples_per_pixel, new_samples_per_pixel)
        }

        Self { nx_ny }
    }
}

impl Sampler for StratifiedSampler {
    fn camera_samples(
        &self,
        p_raster: glam::Vec2,
        rng: &mut rand::rngs::StdRng,
    ) -> Vec<CameraSample> {
        let mut samples = Vec::with_capacity(self.nx_ny.pow(2));

        for y in 0..self.nx_ny {
            for x in 0..self.nx_ny {
                let grid_offset = (glam::vec2(x as f32, y as f32)
                    + rand::Rng::gen::<glam::Vec2>(rng))
                    / self.nx_ny as f32;
                let p_film = p_raster + grid_offset;

                let p_lens = rand::Rng::gen::<glam::Vec2>(rng);

                let sample = CameraSample { p_film, p_lens };

                samples.push(sample)
            }
        }

        samples
    }
}
