mod modules {
    pub mod banner;
    pub mod texture_buffer;
    pub mod utils;
    pub mod population;
}

use modules::{
    texture_buffer::TextureBuffer,
    utils::COLORS,
    population::Population,
};

use image::open;


fn main() {
    let texture_buffer = TextureBuffer::new("./src/textures");
    let mut target = image::open("./src/renders/atlas.png").unwrap().to_rgba();
    let mut population = Population::new(&texture_buffer, &mut target, 300);
    population.calculate_fitness();
    let top_banners = population.elitist_selection(5);
    for (i, banner) in top_banners.iter().enumerate() {
        banner.save(&format!("./src/renders/banner_{}_RMSE_{}.png", i, banner.fitness.unwrap()));
    }
}