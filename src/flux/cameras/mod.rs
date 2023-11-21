mod perspective;

pub use perspective::*;

use super::{ray::Ray, samplers::CameraSample};

pub trait Camera {
    fn ray(&self, sample: &CameraSample) -> Ray;
    fn resoltuion(&self) -> glam::UVec2;
}
