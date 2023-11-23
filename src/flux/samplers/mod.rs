mod stratified;
mod uniform;

pub use stratified::*;


pub trait Sampler {
    fn camera_samples(
        &self,
        p_raster: glam::Vec2,
        rng: &mut rand::rngs::StdRng,
    ) -> Vec<CameraSample>;
}

pub struct CameraSample {
    pub p_film: glam::Vec2,
    pub p_lens: glam::Vec2,
}
