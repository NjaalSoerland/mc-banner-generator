use image::{Rgba, ImageBuffer, GenericImageView};
use std::collections::HashMap;
use std::fs;

pub struct TextureBuffer {
    pub base: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub textures: HashMap<String, ImageBuffer<Rgba<u8>, Vec<u8>>>,
}

impl TextureBuffer {
    pub fn new(texture_dir: &str) -> Self {
        let base = Self::load_texture("base", texture_dir);
        let mut textures = HashMap::new();

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

        TextureBuffer { base, textures }
    }

    fn load_texture(name: &str, dir: &str) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let img = image::open(format!("{}/{}.png", dir, name)).unwrap().to_rgba8();
        img.view(1, 1, 20, 40).to_image()
    }
}
