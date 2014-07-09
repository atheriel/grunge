/*
    This file is part of grunge, a coherent noise generation library.
*/

extern crate test;
extern crate cgmath;
extern crate grunge;

use cgmath::vector::Vector2;
use grunge::primitives::snoise_2d;

#[bench]
fn bench_simplex_noise_2d(b: &mut test::Bencher) {
    b.iter(|| {
    	snoise_2d(Vector2::new(0.05, 0.05), 0u)
    });
}
