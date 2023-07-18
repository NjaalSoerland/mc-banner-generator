use super::texture_buffer::TextureBuffer;
use image::{Rgba, ImageBuffer, imageops};

pub struct Banner<'a> {
    pub base_color: Rgba<u8>,
    pub layers: Vec<(Rgba<u8>, String)>,
    pub render: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub texture_buffer: &'a TextureBuffer,
    pub fitness: Option<f64>,
}

impl<'a> Banner<'a> {
    pub fn new(base_color: Rgba<u8>, layers: Vec<(Rgba<u8>, String)>, texture_buffer: &'a TextureBuffer) -> Self {
        let mut banner = Self {
            base_color,
            layers,
            render: ImageBuffer::new(20, 40),
            texture_buffer,
            fitness: None,
        };
        banner.render();
        banner
    }

    pub fn render(&mut self) {
        self.render = self.colorize_texture(self.texture_buffer.base.clone(), self.base_color);

        for (color, texture_name) in &self.layers { 
            let texture = self.texture_buffer.textures.get(texture_name).unwrap().clone();
            let colored_texture = self.colorize_texture(texture, *color);
            imageops::overlay(&mut self.render, &colored_texture, 0, 0);
        }
    }

    pub fn calculate_fitness(&mut self, target: &ImageBuffer<Rgba<u8>, Vec<u8>>) {
        self.fitness = Some(self.render.pixels().zip(target.pixels()).map(|(p1, p2)| {
            let diff_r = p1[0] as f64 - p2[0] as f64;
            let diff_g = p1[1] as f64 - p2[1] as f64;
            let diff_b = p1[2] as f64 - p2[2] as f64;
            let diff_a = p1[3] as f64 - p2[3] as f64;
            diff_r*diff_r + diff_g*diff_g + diff_b*diff_b + diff_a*diff_a
        }).sum::<f64>().sqrt());
    }

    fn colorize_texture(&self, mut texture: ImageBuffer<Rgba<u8>, Vec<u8>>, color: Rgba<u8>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let (width, height) = texture.dimensions();

        for y in 0..height {
            for x in 0..width {
                let pixel = texture.get_pixel(x, y);

                let intensity = pixel[0] as f32 / 255.0;

                let new_pixel = Rgba([
                    (color[0] as f32 * intensity) as u8,
                    (color[1] as f32 * intensity) as u8,
                    (color[2] as f32 * intensity) as u8,
                    pixel[3],
                ]);

                texture.put_pixel(x, y, new_pixel);
            }
        }

        texture
    }

    pub fn save(&self, path: &str) {
        self.render.save(path).unwrap();
    }
}
