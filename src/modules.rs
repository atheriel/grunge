/*
    This file is part of grunge, a coherent noise generation library.
*/

//! Re-exports NoiseModule and all types that implement it.
//!
//! This is intended to serve as the primary API for the library.

pub use primitives::NoiseModule;
pub use modifiers::Modifiable;
pub use fractal::{
    PinkNoise,
    BillowNoise
};
pub use geometry::{
    ConstNoise,
    CheckerboardNoise,
    CylinderNoise,
    SphereNoise,
    FunctionNoise
};
pub use modifiers::{
    ClampedNoise,
    ScaledBiasedNoise,
    TranslatedNoise,
    RotatedNoise,
    ModifierNoise
};
