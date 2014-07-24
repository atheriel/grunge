/*
    This file is part of grunge, a coherent noise generation library.
*/

extern crate grunge;
extern crate image;

use std::io::File;
use image::GenericImage;

use grunge::vectors::Vector2;
use grunge::modules::{NoiseModule, Modifiable, PinkNoise};

#[deriving(Clone)]
struct GaussNoise {
    pub variance: f32
}

impl NoiseModule for GaussNoise {
    #[inline]
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str> {
        Ok(1.0 / (2.0 * Float::pi() * self.variance) * (- 0.5 *
            self.variance.recip() * (v.x.powi(2) + v.y.powi(2)))
            .exp()
        )
    }
}

impl Modifiable for GaussNoise {}

fn main() {
    let gauss = GaussNoise { variance: 0.15 };
    let scaled1 = gauss.scalebias(0.85, -0.2);
    let final1 = scaled1.clamp(0.0, 1.0);
    let noise = PinkNoise::new(0u);
    let scaled2 = noise.scalebias(0.15, 0.5);
    let final2 = scaled2.clamp(0.0, 1.0);

    // Create the image buffer
    let mut imbuf = image::ImageBuf::new(128, 128);

    // Write a block of 128x128 pixels to disk
    for y in range(-64i, 64i) {
        for x in range(-64i, 64i) {
            let point = Vector2::new((x as f32) / 100.0, (y as f32) / 100.0);
            let value = final1.generate_2d(point).unwrap() * final2.generate_2d(point).unwrap() * 255.0;
            imbuf.put_pixel((x + 64) as u32, (y + 64) as u32, image::Luma(value as u8));
        }
    }

    // Open a file to dump the image data to
    let file = match File::create(&Path::new("example2.png")) {
        Ok(f) => f,
        Err(e) => fail!("file error: {}", e),
    };

    // Write the image to disk
    let _ = image::ImageLuma8(imbuf).save(file, image::PNG);

    println!("Output image written to example2.png");
}
