mod modules {
    pub mod banner;
    pub mod texture_buffer;
    pub mod utils;
}

use modules::{
    banner::Banner,
    texture_buffer::TextureBuffer,
    utils::COLORS,
};

use image::Rgba;
use rand::Rng;


fn main() {
    let texture_buffer = TextureBuffer::new("./src/textures");

    let mut rng = rand::thread_rng();

    for i in 0..=6 {
        let mut layers = Vec::new();
        let base_color = COLORS[rng.gen_range(0..COLORS.len())].0;
        for _ in 0..i {
            let (color, _) = COLORS[rng.gen_range(0..COLORS.len())];
            let texture_name = texture_buffer.textures.keys().nth(rng.gen_range(0..texture_buffer.textures.len())).unwrap().to_string();
            layers.push((color, texture_name));
        }

        let banner = Banner::new(base_color, &layers, &texture_buffer);
        banner.save_render(&format!("./src/renders/output_{}.png", i));
    }
}
