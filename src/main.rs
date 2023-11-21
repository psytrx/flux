use crate::example_scenes::{load_example_scene, ExampleScene};

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
        log::debug!("rendering film...");
        measure_time::trace_time!("rendering film");
        flux::render_film(&scene, 32).gamma_corrected(2.0)
    };

    let img: image::Rgb32FImage = {
        measure_time::trace_time!("converting film to image");
        film.into()
    };

    {
        measure_time::trace_time!("saving image");
        img.save("./output/output.exr")?;
    }

    Ok(())
}
