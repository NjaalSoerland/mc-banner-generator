use super::banner::Banner;
use super::texture_buffer::TextureBuffer;
use image::{ImageBuffer, Rgba};
use rand::Rng;
use rand::distributions::{Distribution, WeightedIndex};
use crate::COLORS;
use rayon::prelude::*;

pub struct Population<'a> {
    pub banners: Vec<Banner<'a>>,
    target: &'a ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl<'a> Population<'a> {
    pub fn new(texture_buffer: &'a TextureBuffer, target: &'a ImageBuffer<image::Rgba<u8>, Vec<u8>>, size: usize) -> Self {
        let evaluated_colors = Self::evaluate_base_colors(texture_buffer, target);
    
        let weights: Vec<f64> = evaluated_colors.iter().map(|&(_, fitness)| 1.0 / fitness).collect();
        let dist = WeightedIndex::new(&weights).unwrap();
    
        let mut banners = Vec::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            let layers = (0..rng.gen_range(0..7))
                .map(|_| {
                    let color = evaluated_colors[dist.sample(&mut rng)].0;
                    let texture_name = texture_buffer.textures.keys().nth(rng.gen_range(0..texture_buffer.textures.len())).unwrap().to_string();
                    (color, texture_name)
                })
                .collect::<Vec<_>>();
            
            let base_color = evaluated_colors[dist.sample(&mut rng)].0;
            let banner = Banner::new(base_color, layers, texture_buffer);
            banners.push(banner);
        }
        Self { banners, target }
    }

    // -------------------------------------------- Utils --------------------------------------------

    fn evaluate_base_colors(texture_buffer: &'a TextureBuffer, target: &'a ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> Vec<(Rgba<u8>, f64)> {
        COLORS.iter().map(|&(color, _)| {
            let mut banner = Banner::new(color, vec![], texture_buffer);
            banner.calculate_fitness(target);
            (color, banner.fitness.unwrap())
        })
        .collect()
    }

    pub fn calculate_fitness(&mut self) {
        self.banners.par_iter_mut().for_each(|banner| {
            banner.calculate_fitness(self.target);
        });
    }

    // -------------------------------------------- Selection --------------------------------------------

    pub fn elitist_selection(&mut self, count: usize) -> Vec<Banner<'a>> {
        let mut sorted_banners = self.banners.clone();
        sorted_banners.sort_unstable_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
        sorted_banners.truncate(count);
        sorted_banners
    }

    pub fn fitness_proportionate_selection(&self) -> &Banner<'a> {
        let total_fitness: f64 = self.banners.iter().map(|b| 1.0 / b.fitness.unwrap()).sum();

        let mut rng = rand::thread_rng();
        let mut threshold: f64 = rng.gen_range(0.0..total_fitness);

        for banner in &self.banners {
            threshold -= 1.0 / banner.fitness.unwrap();
            if threshold <= 0.0 {
                return banner;
            }
        }

        &self.banners[0]
    }

}
