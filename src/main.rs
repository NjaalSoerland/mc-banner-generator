mod modules {
    pub mod banner;
    pub mod texture_buffer;
    pub mod utils;
    pub mod population;
    pub mod ga;
}

use modules::{
    texture_buffer::TextureBuffer,
    utils::COLORS,
    population::Population,
    ga::GA,
};

use image::open;
use std::time::Instant;


fn main() {
    let texture_buffer = TextureBuffer::new("./src/textures");
    let mut target = open("./src/renders/atlas.png").unwrap().to_rgba();
    
    let pop_size = 300;
    let generations = 100;

    let start = Instant::now();

    let mut ga = GA::new(&texture_buffer, &mut target, pop_size, 0.8);
    ga.run(generations);
    let best_banner = ga.best();
    best_banner.save("./src/renders/best.png");

    println!("Time elapsed: {:?}", start.elapsed());
}