/*
    This file is part of grunge, a coherent noise generation library.
*/

extern crate grunge;
extern crate image;

use std::default::Default;
use std::io::File;
use image::GenericImage;

use grunge::vectors::Vector2;
use grunge::modules::{
    NoiseModule,
    Modifiable,
    PinkNoise,
    BillowNoise,
    RidgedMultifractalNoise
};

fn create_png(noise: &Modifiable, filename: &'static str) {
    // Scale noise (which has the range [-1, 1]) to the range [0, 1]
    let final = noise.scalebias(0.5, 0.5).clamp(0.0, 1.0);

    // Create the image buffer
    let mut imbuf = image::ImageBuf::new(200, 200);

    // Write a block of 200x200 pixels to the buffer
    for y in range(0, 200u32) {
        for x in range(0, 200u32) {
            let point = Vector2::new(x as f32, y as f32);
            let value = final.generate_2d(point).unwrap() * 255.0;
            imbuf.put_pixel(x, y, image::Luma(value as u8));
        }
    }

    // Open a file to dump the image data to
    let file = match File::create(&Path::new(filename)) {
        Ok(f) => f,
        Err(e) => fail!("File error: {}", e),
    };

    // Write the image to disk
    let _ = image::ImageLuma8(imbuf).save(file, image::PNG);
}

fn main() {
    let pink = PinkNoise { seed: 1u, frequency: 0.01, .. Default::default() };
    let billow = BillowNoise { seed: 1u, frequency: 0.01, .. Default::default() };
    let ridged = RidgedMultifractalNoise { seed: 1u, frequency: 0.01, .. Default::default() };

    create_png(&pink, "pink.png");
    create_png(&billow, "billow.png");
    create_png(&ridged, "ridged.png");
}
