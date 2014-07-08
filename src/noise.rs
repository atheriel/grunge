/*
    This file is part of rs-noise, a procedural noise generation library.
*/

#![crate_id   = "noise#0.1-pre"]
#![comment = "A procedural noise generation library."]
#![crate_type = "dylib"]
#![crate_type = "rlib"]
// #![no_std] // One day...

// #![warn(missing_doc)]
#![feature(globs)]
#![feature(macro_rules)]

extern crate cgmath;

pub mod common;
pub mod primitives;
pub mod fractal;
