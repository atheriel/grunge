/*
    This file is part of grunge, a coherent noise generation library.
*/

//! This module contains types for generating noise with geometric forms.
//!
//! While these types do not produce "coherent" noise per se, they can be very
//! useful when constructing complex, multi-component noise.

use std::default::Default;
use cgmath::vector::{Vector2, Vector, EuclideanVector};

use common::NoiseModule;
use modifiers::Modifiable;

/// ConstNoise will generate the same value of noise for any input coordinate.
pub struct ConstNoise {
    /// The value of noise to output.
	pub value: f32
}

impl ConstNoise {
    /// Create a new ConstNoise with the given value.
	pub fn new(value: f32) -> ConstNoise {
		ConstNoise { value: value }
	}
}

impl NoiseModule for ConstNoise {
	#[inline]
	fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str> {
		Ok(self.value)
	}
}

impl Modifiable for ConstNoise {}

/// CheckerboardNoise will generate a checkerboard pattern.
pub struct CheckerboardNoise;

impl NoiseModule for CheckerboardNoise {
    #[inline]
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str> {
        if ((v.x as int) & 1 ^ (v.y as int) & 1) != 0
            { Ok(-1.0) } else { Ok(1.0) }
    }
}

impl Modifiable for CheckerboardNoise {}

/// CylinderNoise will generate noise around concentric cylinders whose base is
/// in the x-y plane.
pub struct CylinderNoise {
    /// The frequency of the cylinders. This value can be used to effectively
    /// change how far the rings are apart, but really just scales the input.
    pub frequency: f32
}

impl CylinderNoise {
    /// Create a new CylinderNoise with the given frequency.
    pub fn new(frequency: f32) -> CylinderNoise {
        CylinderNoise { frequency: frequency }
    }
}

impl NoiseModule for CylinderNoise {
    #[inline]
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str> {
        let fract = v.mul_s(self.frequency).length().fract();
        Ok(1.0 - fract.min(1.0 - fract) * 4.0)
    }
}

impl Modifiable for CylinderNoise {}

/// This implementation is stubbed until there is 3D support.
pub type SphereNoise = CylinderNoise;

/// FunctionNoise allows the use of an arbitrary function to generate noise.
///
/// Note that you must call it using `mut_generate_*` instead of the usual
/// `generate_*` methods, due to the nature of Rust's closures.
///
/// ## Example
///
/// Implementing a "Gaussian" Noise generator.
///
/// ```rust
/// let gauss = FunctionNoise::new(|x, y| {
///     Ok(1 / (2 * Float::pi()) * (- 0.5 * (x^2 + y^2)).exp())
/// });
/// println!("{}", gauss.mut_generate_2d(Vector2::unit_x()));
/// ```
pub struct FunctionNoise<'a> {
    /// The function which maps points to a noise value.
    pub func: |x: f32, y: f32|: 'a -> Result<f32, &str>
}

impl<'a> FunctionNoise<'a> {
    /// Create a new FunctionNoise with the given function.
    #[inline]
    pub fn new(func: |x: f32, y: f32|: 'a -> Result<f32, &str>)
        -> FunctionNoise<'a> { FunctionNoise { func: func } }

}

impl<'a> NoiseModule for FunctionNoise<'a> {
    #[inline]
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str> {
        Err("Closures require a mutable environment. Use mut_generate_2d().")
    }

    #[inline]
    fn mut_generate_2d(&mut self, v: Vector2<f32>) -> Result<f32, &str> {
        (self.func)(v.x, v.y)
    }
}

impl<'a> Modifiable for FunctionNoise<'a> {}
