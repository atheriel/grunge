/*
    This file is part of grunge, a coherent noise generation library.
*/

use cgmath::vector::Vector2;

pub trait NoiseModule {
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str>;
}

pub trait SeededModule {
    fn new(seed: uint) -> Self;
}
