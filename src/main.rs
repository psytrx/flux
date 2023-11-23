use flux::{Denoiser, Film};
use image::Rgb32FImage;

use crate::{
    example_scenes::{load_example_scene, ExampleScene},
    flux::samplers::StratifiedSampler,
};

mod example_scenes;
mod flux;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let scene = {
        log::debug!("loading scene...");
        measure_time::trace_time!("loading scene");
        load_example_scene(ExampleScene::MaterialDemo)
    };

    let film = {
        let sampler = StratifiedSampler::new(4_usize.pow(2));

        log::debug!("rendering film...");
        measure_time::trace_time!("rendering film");
        flux::render_film(&scene, sampler, 32).gamma_corrected(2.0)
    };

    {
        let raw: Rgb32FImage = film.clone().into();
        raw.save("./output/output_raw.exr")?;
    };

    let denoised = {
        log::debug!("denoising film...");
        measure_time::trace_time!("denoising film");

        let albedo = Film::new(film.resolution);
        let albedo_img: Rgb32FImage = albedo.clone().into();
        albedo_img.save("./output/output_albedo.exr")?;

        let normal = Film::new(film.resolution);
        let normal_img: Rgb32FImage = normal.clone().into();
        normal_img.save("./output/output_normal.exr")?;

        let denoiser = Denoiser::new(film.resolution, &albedo, &normal);
        denoiser.denoise(&film)
    };

    {
        let raw: Rgb32FImage = denoised.clone().into();
        raw.save("./output/output_denoised.exr")?;
    }

    Ok(())
}
