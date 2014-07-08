extern crate test;
extern crate noise;
extern crate cgmath;

use std::io::{File, Truncate, Write};
use cgmath::vector::Vector2;

use noise::common::NoiseModule;
use noise::fractal::PinkNoise;

fn clamp(val: f32, min: f32, max: f32) -> f32 {
    if val > max { max } else if val < min { min } else { val }
}

fn main() {
    let noise = PinkNoise { seed: 0, frequency: 1.0, persistence: 0.5, lacunarity: 2.0, octaves: 6 };
    let p = Path::new("example1.pgm");

    let mut file = match File::open_mode(&p, Truncate, Write) {
        Ok(f) => f,
        Err(e) => fail!("file error: {}", e),
    };

    // Write the PGM header first
    let _ = file.write_str(format!("P5\n{0} {1}\n{2}\n", 500u, 500u, 255u).as_slice());

    // Write a sample of 500x500 pixels to disk
    for y in range(-250i, 250i) {
        for x in range(-250i, 250i) {
            let tmp = clamp(noise.generate_2d(Vector2::new((x as f32) / 100.0, (y as f32) / 100.0)).unwrap() * 0.15 + 0.5, 0.0, 1.0) * 255.0;
            let _ = file.write_u8(tmp as u8);
        }
    }

    println!("--- Output image written to example1.pgm");
}
