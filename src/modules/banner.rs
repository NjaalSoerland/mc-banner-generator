use super::texture_buffer::TextureBuffer;
use image::{Rgba, ImageBuffer, imageops};
use rand::Rng;
use rand::prelude::SliceRandom;
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
        self.render = self.colorize_texture(self.texture_buffer.base.clone(), self.base_color);

        for (color, texture_name) in &self.layers { 
            let texture = self.texture_buffer.textures.get(texture_name).unwrap().clone();
            let colored_texture = self.colorize_texture(texture, *color);
            imageops::overlay(&mut self.render, &colored_texture, 0, 0);
        }
    }

    fn colorize_texture(&self, mut texture: ImageBuffer<Rgba<u8>, Vec<u8>>, color: Rgba<u8>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let (width, height) = texture.dimensions();

        for y in 0..height {
            for x in 0..width {
                let pixel = texture.get_pixel(x, y);

                let intensity = pixel[0] as f32 / 255.0;

                let new_pixel = Rgba([
                    (color[0] as f32 * intensity) as u8,
                    (color[1] as f32 * intensity) as u8,
                    (color[2] as f32 * intensity) as u8,
                    pixel[3],
                ]);

                texture.put_pixel(x, y, new_pixel);
            }
        }

        texture
    }

    pub fn save(&self, path: &str) {
        self.render.save(path).unwrap();
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
