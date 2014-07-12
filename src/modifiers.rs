/*
    This file is part of grunge, a coherent noise generation library.
*/

use cgmath::vector::Vector2;

use common::NoiseModule;

pub trait Modifiable : NoiseModule {
    fn clamp<'a>(&'a self, min: f32, max: f32) -> ClampedNoise<'a> {
        ClampedNoise { source: self, min: min, max: max }
    }

    fn scalebias<'a>(&'a self, scale: f32, bias: f32) -> ScaledBiasedNoise<'a> {
        ScaledBiasedNoise { source: self, scale: scale, bias: bias }
    }
}

pub struct ClampedNoise<'a> {
	pub source: &'a NoiseModule,
	pub min: f32,
	pub max: f32,
}

impl<'a> ClampedNoise<'a> {
    pub fn new(source: &'a NoiseModule) -> ClampedNoise<'a> {
        ClampedNoise { source: source, min: -1.0, max: 1.0 }
    }
}

impl<'a> NoiseModule for ClampedNoise<'a> {
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str> {
        match self.source.generate_2d(v) {
            Ok(val) => if val > self.max { Ok(self.max) }
                       else if val < self.min { Ok(self.min) }
                       else { Ok(val) },
            err => err
        }
    }
}

impl<'a> Modifiable for ClampedNoise<'a> {}

pub struct ScaledBiasedNoise<'a> {
	pub source: &'a NoiseModule,
	pub scale: f32,
	pub bias: f32,
}

impl<'a> ScaledBiasedNoise<'a> {
    pub fn new(source: &'a NoiseModule) -> ScaledBiasedNoise<'a> {
        ScaledBiasedNoise { source: source, scale: 1.0, bias: 0.0 }
    }
}

impl<'a> NoiseModule for ScaledBiasedNoise<'a> {
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str> {
        match self.source.generate_2d(v) {
            Ok(val) => Ok(val * self.scale + self.bias),
            err => err
        }
    }
}

impl<'a> Modifiable for ScaledBiasedNoise<'a> {}
