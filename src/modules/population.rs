use super::banner::Banner;
use super::texture_buffer::TextureBuffer;
use crate::COLORS;
use image::{ImageBuffer, Rgba};
use rand::distributions::{Distribution, WeightedIndex};
use rand::Rng;
use rayon::prelude::*;

pub struct Population<'a> {
    pub banners: Vec<Banner<'a>>,
    target: &'a ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl<'a> Population<'a> {
    pub fn new(
        texture_buffer: &'a TextureBuffer,
        target: &'a ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        size: usize,
    ) -> Self {
        let evaluated_colors: Vec<(Rgba<u8>, f64)> =
            Self::evaluate_base_colors(texture_buffer, target);

        let weights: Vec<f64> = evaluated_colors
            .iter()
            .map(|&(_, fitness)| 1.0 / fitness)
            .collect();
        let dist: WeightedIndex<f64> = WeightedIndex::new(&weights).unwrap();

        let mut banners: Vec<Banner<'_>> = Vec::with_capacity(size);
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        for _ in 0..size {
            let layers: Vec<(Rgba<u8>, String)> = (0..rng.gen_range(0..7))
                .map(|_| {
                    let color: Rgba<u8> = evaluated_colors[dist.sample(&mut rng)].0;
                    let texture_name: String = texture_buffer
                        .textures
                        .keys()
                        .nth(rng.gen_range(0..texture_buffer.textures.len()))
                        .unwrap()
                        .to_string();
                    (color, texture_name)
                })
                .collect::<Vec<_>>();

            let base_color: Rgba<u8> = evaluated_colors[dist.sample(&mut rng)].0;
            let banner: Banner<'_> = Banner::new(base_color, layers, texture_buffer);
            banners.push(banner);
        }
        Self { banners, target }
    }

    // -------------------------------------------- Utils --------------------------------------------

    fn evaluate_base_colors(
        texture_buffer: &'a TextureBuffer,
        target: &'a ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    ) -> Vec<(Rgba<u8>, f64)> {
        COLORS
            .iter()
            .map(|&(color, _)| {
                let mut banner: Banner<'_> = Banner::new(color, vec![], texture_buffer);
                banner.calculate_fitness(target);
                (color, banner.fitness.unwrap())
            })
            .collect()
    }

    pub fn calculate_fitness(&mut self) {
        self.banners
            .par_iter_mut()
            .for_each(|banner: &mut Banner<'_>| {
                banner.calculate_fitness(self.target);
            });
    }

    // -------------------------------------------- Selection --------------------------------------------

    pub fn elitist_selection(&mut self, count: usize) -> Vec<Banner<'a>> {
        let mut sorted_banners: Vec<Banner<'_>> = self.banners.clone();
        sorted_banners.sort_unstable_by(|a: &Banner<'_>, b: &Banner<'_>| {
            a.fitness.partial_cmp(&b.fitness).unwrap()
        });
        sorted_banners.truncate(count);
        sorted_banners
    }

    pub fn fitness_proportionate_selection(&self) -> &Banner<'a> {
        let total_fitness: f64 = self
            .banners
            .iter()
            .map(|b: &Banner<'_>| 1.0 / b.fitness.unwrap())
            .sum();

        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
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
