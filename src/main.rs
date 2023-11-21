mod flux;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let resolution = glam::uvec2(512, 512);

    let img = {
        measure_time::trace_time!("drawing image");
        flux::render_image(resolution)
    };

    {
        measure_time::trace_time!("saving image");
        img.save("./output/output.exr")?;
    }

    Ok(())
}
