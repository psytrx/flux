fn main() -> anyhow::Result<()> {
    let resolution = glam::uvec2(512, 512);
    let img = image::Rgb32FImage::from_fn(resolution.x, resolution.y, |x, y| {
        let r = x as f32 / resolution.x as f32;
        let g = y as f32 / resolution.y as f32;
        let b = 0.0;
        image::Rgb([r, g, b])
    });
    img.save("./output/output.exr")?;
    Ok(())
}
