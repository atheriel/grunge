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
