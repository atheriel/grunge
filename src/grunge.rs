/*
    This file is part of grunge, a coherent noise generation library.
*/

#![crate_name   = "grunge"]
#![comment = "A coherent noise generation library."]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![feature(globs)]
#![feature(macro_rules)]

extern crate cgmath;

pub mod modules;

pub mod common;
pub mod primitives;
pub mod fractal;
pub mod geometry;
pub mod modifiers;
