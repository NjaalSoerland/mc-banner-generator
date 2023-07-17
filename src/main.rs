mod modules {
    pub mod banner;
    pub mod texture_buffer;
}

use modules::{
    banner::Banner,
    texture_buffer::TextureBuffer,
};
use image::Rgba;
use rand::Rng;

const COLORS: [(Rgba<u8>, &str); 16] = [
    (Rgba([25, 25, 25, 255]), "Black"),
    (Rgba([76, 76, 76, 255]), "Dark Grey"),
    (Rgba([153, 153, 153, 255]), "Grey"),
    (Rgba([255, 255, 255, 255]), "White"),
    (Rgba([242, 127, 165, 255]), "Pink"),
    (Rgba([178, 76, 216, 255]), "Magenta"),
    (Rgba([127, 63, 178, 255]), "Purple"),
    (Rgba([51, 76, 178, 255]), "Blue"),
    (Rgba([76, 127, 153, 255]), "Cyan"),
    (Rgba([102, 153, 216, 255]), "Light Blue"),
    (Rgba([102, 127, 51, 255]), "Green"),
    (Rgba([127, 204, 25, 255]), "Lime"),
    (Rgba([229, 229, 51, 255]), "Yellow"),
    (Rgba([216, 127, 51, 255]), "Orange"),
    (Rgba([102, 76, 51, 255]), "Brown"),
    (Rgba([153, 51, 51, 255]), "Red"),
];

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
