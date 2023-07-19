// File: src/modules/ga.rs
use super::population::Population;
use super::banner::Banner;
use super::texture_buffer::TextureBuffer;
use rayon::prelude::*;

pub struct GA<'a> {
    population: Population<'a>,
    texture_buffer: &'a TextureBuffer,
    best_individual: Option<Banner<'a>>,
}

impl<'a> GA<'a> {
    pub fn new(texture_buffer: &'a TextureBuffer, target: &'a mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, pop_size: usize) -> Self {
        let population = Population::new(texture_buffer, target, pop_size);
        Self {
            population,
            texture_buffer,
            best_individual: None,
        }
    }

    pub fn run(&mut self, generations: usize) {
        for _ in 0..generations {
            self.population.calculate_fitness();
            let best = self.population.elitist_selection(1)[0].clone();
            
            if self.best_individual.is_none() || best.fitness.unwrap() < self.best_individual.as_ref().unwrap().fitness.unwrap() {
                self.best_individual = Some(best.clone());
            }

            let new_banners: Vec<Banner> = (0..self.population.banners.len())
                .into_par_iter()
                .map(|_| {
                    let parent1 = self.population.fitness_proportionate_selection();
                    let parent2 = self.population.fitness_proportionate_selection();
                    let mut child = parent1.crossover(parent2);
                    child.mutate();
                    child
                })
                .collect();
            
            self.population.banners = new_banners;
            self.population.banners.push(best);
        }
    }

    pub fn best(&self) -> &Banner {
        self.best_individual.as_ref().unwrap()
    }
}
