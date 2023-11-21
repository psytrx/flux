pub fn render_image(resolution: glam::UVec2) -> image::Rgb32FImage {
    image::Rgb32FImage::from_fn(resolution.x, resolution.y, |x, y| {
        let r = x as f32 / resolution.x as f32;
        let g = y as f32 / resolution.y as f32;
        let b = 0.0;
        image::Rgb([r, g, b])
    })
}
