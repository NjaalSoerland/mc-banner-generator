mod modules {
    pub mod banner;
    pub mod ga;
    pub mod population;
    pub mod texture_buffer;
    pub mod utils;
}

use modules::{ga::GA, texture_buffer::TextureBuffer, utils::COLORS};

use image::open;
use std::time::Instant;

fn main() {
    let texture_buffer: TextureBuffer = TextureBuffer::new("./src/textures");
    let mut target: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        open("./src/renders/ent.png").unwrap().to_rgba8();

    let pop_size: usize = 3000;
    let generations: usize = 10;

    let start: Instant = Instant::now();

    let mut ga: GA<'_> = GA::new(&texture_buffer, &mut target, pop_size, 0.8);
    ga.run(generations);
    let best_banner: &modules::banner::Banner<'_> = ga.best();

    best_banner.save("./src/renders/best.png");

    println!("Time elapsed: {:?}", start.elapsed());
}
