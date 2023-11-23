pub struct ImageTexture {
    width: u32,
    height: u32,
    cache: Vec<glam::Vec3>,
}

impl ImageTexture {
    pub fn new(img: image::DynamicImage) -> Self {
        // build a directly accessible color cache
        let (width, height) = image::GenericImageView::dimensions(&img);
        let mut cache = vec![glam::Vec3::ZERO; (width * height) as usize];
        for (x, y, pixel) in image::GenericImageView::pixels(&img) {
            let index = (y * width + x) as usize;
            let color = match pixel {
                image::Rgba([r, g, b, _]) => {
                    glam::vec3((r as f32) / 255.0, (g as f32) / 255.0, (b as f32) / 255.0)
                }
            };
            cache[index] = color;
        }
        Self {
            width,
            height,
            cache,
        }
    }

    pub fn evaluate(&self, uv: glam::Vec2) -> glam::Vec3 {
        // flip y coordinate to image coordinate space
        let uv = glam::vec2(uv.x, 1.0 - uv.y);

        let x = (uv.x * (self.width - 1) as f32) as u32;
        let y = (uv.y * (self.height - 1) as f32) as u32;
        let index = (y * self.width + x) as usize;

        self.cache[index]
    }
}
