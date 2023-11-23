use flux::{integrators::Integrator, Denoiser, Film};
use image::Rgb32FImage;

use crate::{
    example_scenes::{load_example_scene, ExampleScene},
    flux::{
        integrators::{AlbedoIntegrator, NormalIntegrator, PathTracingIntegrator},
        samplers::StratifiedSampler,
    },
};

mod example_scenes;
mod flux;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let scene = {
        log::debug!("loading scene...");
        measure_time::trace_time!("loading scene");
        load_example_scene(ExampleScene::CornellBox)
    };

    let film = {
        let sampler = StratifiedSampler::new(4_usize.pow(2));
        let integrator = PathTracingIntegrator::new(32);

        log::debug!("rendering film...");
        measure_time::trace_time!("rendering film");
        flux::render_film(&scene, sampler, integrator).gamma_corrected(2.0)
    };

    {
        let raw: Rgb32FImage = film.clone().into();
        raw.save("./output/beauty_raw.exr")?;
    };

    let denoised = {
        log::debug!("denoising film...");
        measure_time::trace_time!("denoising film");

        let albedo = {
            measure_time::trace_time!("rendering albedo aux channel");
            let albedo = render_aux_channel(&scene, AlbedoIntegrator::new());
            let albedo_img: Rgb32FImage = albedo.clone().into();
            albedo_img.save("./output/albedo_raw.exr")?;
            albedo
        };

        let normal = {
            measure_time::trace_time!("rendering normal aux channel");
            let normal = render_aux_channel(&scene, NormalIntegrator::new());
            let normal_img: Rgb32FImage = normal.clone().into();
            normal_img.save("./output/normal_raw.exr")?;

            // OIDN expects normal vectors in the range [-1, 1]
            normal.mapped_spectra(|v| 2.0 * v - glam::Vec3::ONE)
        };

        let denoiser = Denoiser::new(film.resolution, &albedo, &normal);

        let albedo_denoised_img: Rgb32FImage = denoiser.albedo_denoised.clone().into();
        albedo_denoised_img.save("./output/albedo_denoised.exr")?;

        let normal_denoised_img: Rgb32FImage = denoiser
            .normal_denoised
            // map the denoised normals back to the range [0, 1]
            .mapped_spectra(|v| 0.5 * (v + glam::Vec3::ONE))
            .into();
        normal_denoised_img.save("./output/normal_denoised.exr")?;

        denoiser.denoise(&film)
    };

    let denoised_img: Rgb32FImage = denoised.clone().into();
    denoised_img.save("./output/beauty_denoised.exr")?;

    Ok(())
}

fn render_aux_channel(scene: &flux::Scene, integrator: impl Integrator) -> Film {
    let sampler = StratifiedSampler::new(1_usize.pow(2));
    flux::render_film(scene, sampler, integrator)
}
