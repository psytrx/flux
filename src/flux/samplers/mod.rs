mod uniform;

pub use uniform::*;

pub trait Sampler {
    fn camera_samples(
        &self,
        p_raster: glam::Vec2,
        rng: &mut rand::rngs::StdRng,
    ) -> Vec<CameraSample>;
}

pub struct CameraSample {
    pub p_film: glam::Vec2,
}
