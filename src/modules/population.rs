// File: src/modules/population.rs
use super::banner::Banner;
use super::texture_buffer::TextureBuffer;
use image::{ImageBuffer, Rgba};
use rand::Rng;
use crate::COLORS;

pub struct Population<'a> {
    pub banners: Vec<Banner<'a>>,
    target: &'a ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl<'a> Population<'a> {
    pub fn new(texture_buffer: &'a TextureBuffer, target: &'a mut ImageBuffer<image::Rgba<u8>, Vec<u8>>, size: usize) -> Self {
        let mut banners = Vec::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            let layers = (0..rng.gen_range(0..7))
                .map(|_| {
                    let color = COLORS[rng.gen_range(0..COLORS.len())].0;
                    let texture_name = texture_buffer.textures.keys().nth(rng.gen_range(0..texture_buffer.textures.len())).unwrap().to_string();
                    (color, texture_name)
                })
                .collect::<Vec<_>>();
            let base_color = COLORS[rng.gen_range(0..COLORS.len())].0;
            let banner = Banner::new(base_color, layers, texture_buffer);
            banners.push(banner);
        }
        Self { banners, target }
    }

    pub fn calculate_fitness(&mut self) {
        for banner in &mut self.banners {
            banner.calculate_fitness(self.target);
        }
    }

    pub fn elitist_selection(&mut self, count: usize) {
        self.banners.sort_unstable_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
        self.banners.truncate(count);
    }
}
