/*
    This file is part of grunge, a coherent noise generation library.
*/

use cgmath::vector::Vector2;

pub trait NoiseModule {
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str>;
    fn mut_generate_2d(&mut self, v: Vector2<f32>) -> Result<f32, &str> {
        self.generate_2d(v)
    }
}

pub trait SeededModule {
    fn new(seed: uint) -> Self;
}
