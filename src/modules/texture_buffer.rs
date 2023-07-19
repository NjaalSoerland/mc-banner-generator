use image::{Rgba, ImageBuffer, GenericImageView};
use std::collections::HashMap;
use crate::COLORS;
use std::fs;

pub struct TextureBuffer {
    pub base: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub textures: HashMap<String, ImageBuffer<Rgba<u8>, Vec<u8>>>,
    pub colored_textures: HashMap<(String, Rgba<u8>), ImageBuffer<Rgba<u8>, Vec<u8>>>,
}

impl TextureBuffer {
    pub fn new(texture_dir: &str) -> Self {
        let base = Self::load_texture("base", texture_dir);
        let mut textures = HashMap::new();
        let mut colored_textures = HashMap::new();

        let entries = fs::read_dir(texture_dir).unwrap();
        for entry in entries {
            let path = entry.unwrap().path();
            if path.is_file() && path.extension().unwrap() == "png" {
                let filename = path.file_stem().unwrap().to_str().unwrap().to_string();
                if filename != "base" {
                    textures.insert(filename.clone(), Self::load_texture(&filename, texture_dir));
                }
            }
        }
    
        TextureBuffer { base, textures, colored_textures }
    }
    

    fn load_texture(name: &str, dir: &str) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let img = image::open(format!("{}/{}.png", dir, name)).unwrap().to_rgba8();
        img.view(1, 1, 20, 40).to_image()
    }

    pub fn get_colored_texture(&mut self, name: &str, color: Rgba<u8>) -> &ImageBuffer<Rgba<u8>, Vec<u8>> {
        if !self.colored_textures.contains_key(&(name.to_string(), color)) {
            if let Some(texture) = self.textures.get(name) {
                let mut colored_texture = texture.clone();
                let (width, height) = colored_texture.dimensions();

                for y in 0..height {
                    for x in 0..width {
                        let pixel = colored_texture.get_pixel(x, y);

                        let intensity = pixel[0] as f32 / 255.0;

                        let new_pixel = Rgba([
                            (color[0] as f32 * intensity) as u8,
                            (color[1] as f32 * intensity) as u8,
                            (color[2] as f32 * intensity) as u8,
                            pixel[3],
                        ]);

                        colored_texture.put_pixel(x, y, new_pixel);
                    }
                }

                self.colored_textures.insert((name.to_string(), color), colored_texture);
            }
        }

        self.colored_textures.get(&(name.to_string(), color)).unwrap()
    }

}
