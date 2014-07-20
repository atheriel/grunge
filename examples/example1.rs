/*
    This file is part of grunge, a coherent noise generation library.
*/

extern crate grunge;

use std::io::{File, Truncate, Write};

use grunge::vectors::Vector2;
use grunge::modules::{NoiseModule, Modifiable, PinkNoise};

fn main() {
    // Create PinkNoise (which has the range [-1, 1]) for the range [0, 1]
    let noise = PinkNoise::new(0u);
    let scaled = noise.scalebias(0.15, 0.5);
    let final = scaled.clamp(0.0, 1.0);

    // Open a file to dump the image data to
    let p = Path::new("example1.pgm");
    let mut file = match File::open_mode(&p, Truncate, Write) {
        Ok(f) => f,
        Err(e) => fail!("file error: {}", e),
    };

    // Write the PGM header to the file
    let _ = file.write_str(format!("P5\n{0} {1}\n{2}\n", 500u, 500u, 255u)
            .as_slice());

    // Write a block of 500x500 pixels to disk
    for y in range(-250i, 250i) {
        for x in range(-250i, 250i) {
            let point = Vector2::new((x as f32) / 100.0, (y as f32) / 100.0);
            let value = final.generate_2d(point).unwrap() * 255.0;
            let _ = file.write_u8(value as u8);
        }
    }

    println!("Output image written to example1.pgm");
}
