/*
    This file is part of grunge, a coherent noise generation library.
*/

//! # Grunge: Noise & Rust
//!
//! Grunge is a library for working with coherent noise, written in Rust. Its
//! design should be familiar to users of [libnoise]
//! (http://libnoise.sourceforge.net/) and [CoherentNoise]
//! (http://forum.unity3d.com/threads/coherentnoise-procedural-generation-library-released.91784/).
//! Unlike earlier libraries, it uses the [Simplex Noise]
//! (http://en.wikipedia.org/wiki/Simplex_noise) algorithm, and should have
//! better performance as a result. Hopefully, it will one day contain more
//! features as well.
//!
//! ## Current Design & Features
//!
//! All currently implemented noise generators are re-exported in the [modules]
//! (modules/index.html) module. At present, only two-dimensional simplex noise
//! is implemented.
//!
//! ## Example
//!
//! The following writes a [PGM](http://en.wikipedia.org/wiki/Portable_graymap)
//! file using the PinkNoise generator.
//!
//! ```ignore
//! extern crate cgmath;
//! extern crate grunge;
//!
//! use std::io::{File, Truncate, Write};
//! use cgmath::vector::Vector2;
//! use grunge::modules::{NoiseModule, PinkNoise};
//!
//! fn main() {
//!     let noise = PinkNoise::new(0u);
//!     let p = Path::new("example.pgm");
//!
//!     let mut file = match File::open_mode(&p, Truncate, Write) {
//!         Ok(f) => f,
//!         Err(e) => fail!("--- File error: {}", e),
//!     };
//!
//!     // Write the PGM header first. P5 is for binary data (i.e. u8).
//!     let _ = file.write_str(format!("P5\n{0} {1}\n{2}\n", 500u, 500u, 255u)
//!             .as_slice());
//!
//!     // Write a sample of 500x500 pixels to the image file
//!     for y in range(-250i, 250i) {
//!         for x in range(-250i, 250i) {
//!             let point = Vector2::new((x as f32) / 100.0, (y as f32) / 100.0);
//!             // Usually fits in [0, 1]
//!             let tmp = noise.generate_2d(point).unwrap() * 0.15 + 0.5;
//!             let _ = file.write_u8((tmp * 255.0) as u8);
//!         }
//!     }
//!
//!     println!("Output image written to example.pgm");
//! }
//! ```

#![crate_name   = "grunge"]
#![comment = "A coherent noise generation library."]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![feature(globs)]
#![feature(macro_rules)]

#![unstable]

extern crate cgmath;

pub mod modules;

pub mod primitives;
pub mod fractal;
pub mod geometry;
pub mod modifiers;
