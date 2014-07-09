/*
    This file is part of grunge, a coherent noise generation library.
*/

//! This module re-exports NoiseModule and all types that implement it.
//!
//! It is intended to serve as the primary API for the library.

pub use common::NoiseModule;
pub use fractal::{PinkNoise, BillowNoise};
pub use geometry::{ConstNoise, CylinderNoise};
