use super::texture_buffer::TextureBuffer;
use image::{Rgba, ImageBuffer, imageops};
use rand::Rng;
use rand::prelude::SliceRandom;
use std::fs::File;
use std::io::Write;
use crate::COLORS;

#[derive(Clone)]
pub struct Banner<'a> {
    pub base_color: Rgba<u8>,
    pub layers: Vec<(Rgba<u8>, String)>,
    pub render: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub texture_buffer: &'a TextureBuffer,
    pub fitness: Option<f64>,
}

impl<'a> Banner<'a> {
    pub fn new(base_color: Rgba<u8>, layers: Vec<(Rgba<u8>, String)>, texture_buffer: &'a TextureBuffer) -> Self {
        let mut banner = Self {
            base_color,
            layers,
            render: ImageBuffer::new(20, 40),
            texture_buffer,
            fitness: None,
        };
        banner.render();
        banner
    }

    // -------------------------------------------- Rendering --------------------------------------------

    pub fn render(&mut self) {
        self.render = self.texture_buffer.get_colored_texture("base", self.base_color).clone();

        for (color, texture_name) in &self.layers { 
            let colored_texture = self.texture_buffer.get_colored_texture(texture_name, *color).clone();
            imageops::overlay(&mut self.render, &colored_texture, 0, 0);
        }
    }

    fn get_color_name(&self, color: Rgba<u8>) -> Option<&'static str> {
        COLORS.iter().find(|&&(c, _)| c == color).map(|&(_, name)| name)
    }

    pub fn save(&self, path: &str) {
        self.render.save(path).unwrap();

        let mut output = String::new();
        output.push_str(&format!("Fitness: {}\n", self.fitness.unwrap_or(0.0)));

        let base_color_name = self.get_color_name(self.base_color).unwrap_or("Unknown");
        output.push_str(&format!("Base Color: {}\n", base_color_name));

        for (color, texture_name) in &self.layers {
            let color_name = self.get_color_name(*color).unwrap_or("Unknown");
            output.push_str(&format!("Layer - Color: {}, Texture: {}\n", color_name, texture_name));
        }

        let mut file = File::create("./src/renders/best.txt").unwrap();
        file.write_all(output.as_bytes()).unwrap();
    }

    // -------------------------------------------- Fitness --------------------------------------------

    pub fn calculate_fitness(&mut self, target: &ImageBuffer<Rgba<u8>, Vec<u8>>) {
        self.fitness = Some(self.render.pixels().zip(target.pixels()).map(|(p1, p2)| {
            let diff_r = p1[0] as f64 - p2[0] as f64;
            let diff_g = p1[1] as f64 - p2[1] as f64;
            let diff_b = p1[2] as f64 - p2[2] as f64;
            let diff_a = p1[3] as f64 - p2[3] as f64;
            diff_r*diff_r + diff_g*diff_g + diff_b*diff_b + diff_a*diff_a
        }).sum::<f64>().sqrt());
    }

    // -------------------------------------------- Mutation --------------------------------------------

    pub fn mutate(&mut self, mutation_rate: f64, rng: &mut impl Rng) {
        if rng.gen::<f64>() < mutation_rate {
            let num = rng.gen_range(0..5);

            match num {
                0 => self.mutate_insert(rng),
                1 => self.mutate_remove(rng),
                2 => self.mutate_shuffle(rng),
                3 => self.mutate_change_color(rng),
                4 => self.mutate_shuffle_color(rng),
                _ => unreachable!(),
            }
        }
    }

    fn mutate_insert(&mut self, rng: &mut impl Rng) {
        if self.layers.len() >= 6 { return; }
        
        let color = COLORS[rng.gen_range(0..COLORS.len())].0;
        let texture_name = self.texture_buffer.textures.keys().nth(rng.gen_range(0..self.texture_buffer.textures.len())).unwrap().to_string();
        let idx = rng.gen_range(0..self.layers.len() + 1);

        self.layers.insert(idx, (color, texture_name));
        self.render();
    }

    fn mutate_remove(&mut self, rng: &mut impl Rng) {
        if self.layers.len() == 0 { return; }

        let idx = rng.gen_range(0..self.layers.len());

        self.layers.remove(idx);
        self.render();
    }

    fn mutate_shuffle(&mut self, rng: &mut impl Rng) {
        self.layers.shuffle(rng);
        self.render();
    }

    fn mutate_change_color(&mut self, rng: &mut impl Rng) {
        let color = COLORS[rng.gen_range(0..COLORS.len())].0;

        if rng.gen_bool(0.5) || self.layers.is_empty() {
            self.base_color = color;
        } else {
            let idx = rng.gen_range(0..self.layers.len());
            self.layers[idx].0 = color;
        }

        self.render();
    }

    fn mutate_shuffle_color(&mut self, rng: &mut impl Rng) {
        let mut colors: Vec<_> = self.layers.iter().map(|&(color, _)| color).collect();
        colors.push(self.base_color);
        colors.shuffle(rng);

        self.base_color = colors.pop().unwrap();
        for (layer, color) in self.layers.iter_mut().zip(colors) {
            layer.0 = color;
        }

        self.render();
    }

    // -------------------------------------------- Crossover --------------------------------------------

    pub fn crossover(&self, other: &Self, rng: &mut impl Rng) -> Banner<'a> {
        let self_idx = rng.gen_range(0..=self.layers.len());
        let other_idx = rng.gen_range(0..=other.layers.len());
    
        let mut new_layers = Vec::with_capacity(self_idx + other.layers.len() - other_idx);
        
        new_layers.extend_from_slice(&self.layers[..self_idx]);
        new_layers.extend_from_slice(&other.layers[other_idx..]);
        
        new_layers.truncate(6);
        let base_color = if rng.gen_bool(0.5) { self.base_color } else { other.base_color };
    
        Banner::new(base_color, new_layers, self.texture_buffer)
    }
}
