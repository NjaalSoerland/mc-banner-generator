use super::banner::Banner;
use super::population::Population;
use super::texture_buffer::TextureBuffer;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

pub struct GA<'a> {
    population: Population<'a>,
    best_individual: Option<Banner<'a>>,
    initial_mutation_rate: f64,
}

impl<'a> GA<'a> {
    pub fn new(
        texture_buffer: &'a TextureBuffer,
        target: &'a mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        pop_size: usize,
        initial_mutation_rate: f64,
    ) -> Self {
        let population: Population<'_> = Population::new(texture_buffer, target, pop_size);
        Self {
            population,
            best_individual: None,
            initial_mutation_rate,
        }
    }

    pub fn run(&mut self, generations: usize) {
        let mut mutation_rate: f64;

        let total_time_selection: AtomicUsize = AtomicUsize::new(0);
        let total_time_crossover: AtomicUsize = AtomicUsize::new(0);
        let total_time_mutation: AtomicUsize = AtomicUsize::new(0);

        self.population.calculate_fitness();
        for generation in 0..generations {
            mutation_rate =
                (1.0 - (generation as f64 / generations as f64)) * self.initial_mutation_rate;

            let new_banners: Vec<Banner> = (0..self.population.banners.len())
                .into_par_iter()
                .map(|_| {
                    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
                    let start_selection: Instant = Instant::now();
                    let parent1: &Banner<'_> = self.population.fitness_proportionate_selection();
                    let parent2: &Banner<'_> = self.population.fitness_proportionate_selection();
                    total_time_selection.fetch_add(
                        start_selection.elapsed().as_micros() as usize,
                        Ordering::Relaxed,
                    );

                    let start_crossover: Instant = Instant::now();
                    let mut child: Banner<'_> = parent1.crossover(parent2, &mut rng);
                    total_time_crossover.fetch_add(
                        start_crossover.elapsed().as_micros() as usize,
                        Ordering::Relaxed,
                    );

                    let start_mutation: Instant = Instant::now();
                    child.mutate(mutation_rate, &mut rng);
                    total_time_mutation.fetch_add(
                        start_mutation.elapsed().as_micros() as usize,
                        Ordering::Relaxed,
                    );

                    child
                })
                .collect();

            self.population.calculate_fitness();

            let best: Banner<'_> = self.population.elitist_selection(1)[0].clone();

            if self.best_individual.is_none()
                || best.fitness.unwrap() < self.best_individual.as_ref().unwrap().fitness.unwrap()
            {
                self.best_individual = Some(best.clone());
            }

            self.population.banners = new_banners;
        }

        println!(
            "Selection time: {:?}",
            std::time::Duration::from_micros(total_time_selection.load(Ordering::Relaxed) as u64)
        );
        println!(
            "Crossover time: {:?}",
            std::time::Duration::from_micros(total_time_crossover.load(Ordering::Relaxed) as u64)
        );
        println!(
            "Mutation time: {:?}",
            std::time::Duration::from_micros(total_time_mutation.load(Ordering::Relaxed) as u64)
        );
    }

    pub fn best(&self) -> &Banner {
        self.best_individual.as_ref().unwrap()
    }
}
