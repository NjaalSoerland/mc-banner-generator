use super::texture_buffer::TextureBuffer;
use image::{Rgba, ImageBuffer, imageops};

pub struct Banner<'a> {
    pub base_color: Rgba<u8>,
    pub layers: &'a [(Rgba<u8>, String)],
    pub render: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub texture_buffer: &'a TextureBuffer,
}

impl<'a> Banner<'a> {
    pub fn new(base_color: Rgba<u8>, layers: &'a [(Rgba<u8>, String)], texture_buffer: &'a TextureBuffer) -> Self {
        let mut banner = Banner { 
            base_color,
            layers,
            render: ImageBuffer::new(20, 40),
            texture_buffer,
        };
        banner.render();
        banner
    }

    pub fn render(&mut self) {
        let base_texture = self.texture_buffer.base.clone();
        self.render = self.colorize_texture(base_texture, self.base_color);

        for &(color, ref texture_name) in self.layers {
            let texture = self.texture_buffer.textures.get(texture_name).unwrap();
            let colored_texture = self.colorize_texture(texture.clone(), color);
            imageops::overlay(&mut self.render, &colored_texture, 0, 0);
        }
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

    pub fn save_render(&self, path: &str) {
        self.render.save(path).unwrap();
    }
}
