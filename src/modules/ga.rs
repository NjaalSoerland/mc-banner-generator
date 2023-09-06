use super::population::Population;
use super::banner::Banner;
use super::texture_buffer::TextureBuffer;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use rand::Rng;
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

        let total_time_selection = AtomicUsize::new(0);
        let total_time_crossover = AtomicUsize::new(0);
        let total_time_mutation = AtomicUsize::new(0);
    
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
                    let mut rng = rand::thread_rng();
                    let start_selection = Instant::now();
                    let parent1 = self.population.fitness_proportionate_selection();
                    let parent2 = self.population.fitness_proportionate_selection();
                    total_time_selection.fetch_add(start_selection.elapsed().as_micros() as usize, Ordering::Relaxed);
    
                    let start_crossover = Instant::now();
                    let mut child = parent1.crossover(parent2, &mut rng);
                    total_time_crossover.fetch_add(start_crossover.elapsed().as_micros() as usize, Ordering::Relaxed);

                    let start_mutation = Instant::now();
                    child.mutate(mutation_rate, &mut rng);
                    total_time_mutation.fetch_add(start_mutation.elapsed().as_micros() as usize, Ordering::Relaxed);
    
                    child
                })
                .collect();
            
            self.population.banners = new_banners;
            self.population.banners.push(best);
        }
    
        println!("Selection time: {:?}", std::time::Duration::from_micros(total_time_selection.load(Ordering::Relaxed) as u64));
        println!("Crossover time: {:?}", std::time::Duration::from_micros(total_time_crossover.load(Ordering::Relaxed) as u64));
        println!("Mutation time: {:?}", std::time::Duration::from_micros(total_time_mutation.load(Ordering::Relaxed) as u64));
    }

    pub fn best(&self) -> &Banner {
        self.best_individual.as_ref().unwrap()
    }
}
