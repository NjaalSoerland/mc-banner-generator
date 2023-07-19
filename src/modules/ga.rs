use super::population::Population;
use super::banner::Banner;
use super::texture_buffer::TextureBuffer;
use rayon::prelude::*;

pub struct GA<'a> {
    population: Population<'a>,
    texture_buffer: &'a TextureBuffer,
    best_individual: Option<Banner<'a>>,
    initial_mutation_rate: f64,
}

impl<'a> GA<'a> {
    pub fn new(texture_buffer: &'a TextureBuffer, target: &'a mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, pop_size: usize, initial_mutation_rate: f64) -> Self {
        let population = Population::new(texture_buffer, target, pop_size);
        Self {
            population,
            texture_buffer,
            best_individual: None,
            initial_mutation_rate,
        }
    }

    pub fn run(&mut self, generations: usize) {
        let mut mutation_rate = 1.0;

        for generation in 0..generations {
            self.population.calculate_fitness();
            let best = self.population.elitist_selection(1)[0].clone();
            
            if self.best_individual.is_none() || best.fitness.unwrap() < self.best_individual.as_ref().unwrap().fitness.unwrap() {
                self.best_individual = Some(best.clone());
            }

            mutation_rate = (1.0 - (generation as f64 / generations as f64)) * self.initial_mutation_rate;

            let new_banners: Vec<Banner> = (0..self.population.banners.len())
                .into_par_iter()
                .map(|_| {
                    let parent1 = self.population.fitness_proportionate_selection();
                    let parent2 = self.population.fitness_proportionate_selection();
                    let mut child = parent1.crossover(parent2);
                    child.mutate(mutation_rate);
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
