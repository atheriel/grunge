/*
    This file is part of grunge, a coherent noise generation library.
*/

extern crate grunge;
extern crate image;

use std::io::File;
use image::GenericImage;

use grunge::vectors::Vector2;
use grunge::modules::{NoiseModule, Modifiable, PinkNoise};

fn main() {
    // Create PinkNoise (which has the range [-1, 1]) for the range [0, 1]
    let noise = PinkNoise::new(0u);
    let scaled = noise.scalebias(0.9, 0.5);
    let final = scaled.clamp(0.0, 1.0);

    // Create the image buffer
    let mut imbuf = image::ImageBuf::new(500, 500);

    // Write a block of 500x500 pixels to disk
    for y in range(0, 500u32) {
        for x in range(0, 500u32) {
            let point = Vector2::new((x as f32) / 100.0, (y as f32) / 100.0);
            let value = final.generate_2d(point).unwrap() * 255.0;
            imbuf.put_pixel(x, y, image::Luma(value as u8));
        }
    }

    // Open a file to dump the image data to
    let file = match File::create(&Path::new("example1.png")) {
        Ok(f) => f,
        Err(e) => fail!("file error: {}", e),
    };

    // Write the image to disk
    let _ = image::ImageLuma8(imbuf).save(file, image::PNG);

    println!("Output image written to example1.png");
}
