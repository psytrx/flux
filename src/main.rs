mod flux;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let resolution = glam::uvec2(512, 512);

    let film = {
        log::debug!("rendering film...");
        measure_time::trace_time!("rendering film");
        flux::render_film(resolution, 32)
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
