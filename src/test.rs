/*
    This file is part of grunge, a coherent noise generation library.
*/

extern crate test;
extern crate cgmath;
extern crate grunge;

use cgmath::vector::Vector2;
use grunge::primitives::snoise_2d;
use grunge::common::NoiseModule;
use grunge::fractal::{PinkNoise, BillowNoise};

#[bench]
fn bench_simplex_noise_2d(b: &mut test::Bencher) {
    b.iter(|| {
    	snoise_2d(Vector2::new(0.05, 0.05), 0u)
    });
}

#[test]
fn test_octave_requirements() {
    let mut pink = PinkNoise::new(0);
    pink.octaves = 1;
    let mut billow = BillowNoise::new(0);
    billow.octaves = 31;

    assert!(pink.generate_2d(Vector2::new(0.05, 0.05)).is_err());
    assert!(billow.generate_2d(Vector2::new(0.05, 0.05)).is_err());
}
