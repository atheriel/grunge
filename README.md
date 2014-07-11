Grunge [![Build Status](https://travis-ci.org/atheriel/grunge.svg?branch=master)](https://travis-ci.org/atheriel/grunge)
======

Grunge is a pseudo-random coherent noise library in the spirit of libnoise, ACL,
and the Coherent Noise Library. Unlike earlier libraries, it uses the [Simplex Noise]
(http://en.wikipedia.org/wiki/Simplex_noise) algorithm, and should have better
performance as a result. Hopefully, it will one day contain more features as well.

The API should be familiar to users of libnoise and the Coherent Noise Library, but is
subject to change without notice at this point in the project -- which is currently in
the very early stages of development.

Example
-------

The following writes a [PGM](http://en.wikipedia.org/wiki/Portable_graymap) file using
the PinkNoise generator.

```rust
use std::io::{File, Truncate, Write};
use cgmath::vector::Vector2;
use grunge::module::{NoiseModule, PinkNoise};

let noise = PinkNoise::new(0u);
let p = Path::new("example.pgm");

let mut file = match File::open_mode(&p, Truncate, Write) {
    Ok(f) => f,
    Err(e) => fail!("--- File error: {}", e),
};

// Write the PGM header first. P5 is for binary data (i.e. u8).
let _ = file.write_str(format!("P5\n{0} {1}\n{2}\n", 500u, 500u, 255u).as_slice());
    
// Write a sample of 500x500 pixels to the image file
for y in range(-250i, 250i) {
    for x in range(-250i, 250i) {
        let point = Vector2::new((x as f32) / 100.0, (y as f32) / 100.0);
        let tmp = noise.generate_2d(point).unwrap() * 0.15 + 0.5; // Usually fits in [0, 1]
        let _ = file.write_u8((tmp * 255.0) as u8);
    }
}

println!("--- Output image written to example.pgm");
```

In case you can't open the PGM format, ImageMagick's `convert example.pgm example.png`
may do the trick.

Planned Features
----------------

* Simplex noise primitives in 2D, 3D, and 4D.
* "Fractal" noise types: pink noise, billow niose, and ridged multifractal noise.
* Voronoi noise types
* Geometric noise types: sphere, cylinder, aribtrary functions.
* Modifier types: add, subtract, multiply, turbulence, and so on.
* Helpful utilities for generating images, textures, and so on.
* Paralellization when it improves performance.
* Many examples.
* Language bindings?
