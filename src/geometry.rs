/*
    This file is part of grunge, a coherent noise generation library.
*/

//! Types for generating noise with geometric forms.
//!
//! While these types do not produce "coherent" noise per se, they can be very
//! useful when constructing complex, multi-component noise.

use cgmath::vector::{Vector2, Vector, EuclideanVector};

use primitives::NoiseModule;
use modifiers::Modifiable;

/// ConstNoise will generate the same value of noise for any input coordinate.
///
/// ## Example
///
/// ```rust
/// extern crate grunge;
///
/// use grunge::vectors::Vector2;
/// use grunge::modules::{NoiseModule, ConstNoise};
///
/// fn main() {
///     let noise = ConstNoise::new(5.0);
///     assert_eq!(noise.generate_2d(Vector2::new(101.26, -38.9)),
///                noise.generate_2d(Vector2::new(-26.0, 0.0)));
/// }
/// ```
#[stable]
#[deriving(Clone, PartialEq)]
pub struct ConstNoise {
    /// The value of noise to output.
	pub value: f32
}

#[stable]
impl ConstNoise {
    /// Create a new ConstNoise with the given value.
	pub fn new(value: f32) -> ConstNoise {
		ConstNoise { value: value }
	}
}

impl NoiseModule for ConstNoise {
	#[allow(unused_variable)]
	#[inline]
	fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str> {
		Ok(self.value)
	}
}

impl Modifiable for ConstNoise {}

/// CheckerboardNoise will generate a checkerboard pattern.
#[deprecated = "This type will soon be removed."]
#[deriving(Clone)]
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
///
/// ## Example
///
/// Demonstration of changing the frequency of the cylinders.
///
/// ```rust
/// extern crate grunge;
///
/// use grunge::vectors::Vector2;
/// use grunge::modules::{NoiseModule, CylinderNoise};
///
/// fn main() {
///     let noise1 = CylinderNoise::new(1.0);
///     let noise5 = CylinderNoise::new(5.0);
///     assert_eq!(noise1.generate_2d(Vector2::new(1.0, 0.0)),
///                noise5.generate_2d(Vector2::new(-5.0, 0.0)));
/// }
/// ```
#[deriving(Clone, PartialEq)]
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

/// Functions applicable for passing to FunctionNoise.
pub type FunctionNoiseFunction = fn(x: f32, y: f32) -> Result<f32, &str>;

/// FunctionNoise allows the use of an arbitrary function to generate noise.
///
/// ## Example
///
/// Implementing a "Gaussian" (Multivariate Normal) Noise generator.
///
/// ```rust
/// extern crate grunge;
///
/// use grunge::vectors::Vector2;
/// use grunge::modules::{NoiseModule, FunctionNoise};
///
/// fn gaussian(x: f32, y: f32) -> Result<f32, &str> {
///     Ok(1.0 / (2.0 * Float::pi()) * (- 0.5 * (x.powi(2) + y.powi(2))).exp())
/// }
///
/// fn main() {
///     let gauss = FunctionNoise::new(&gaussian);
///     println!("{}", gauss.generate_2d(Vector2::new(1.0, 1.0)));
/// }
/// ```
#[experimental]
pub struct FunctionNoise<'a> {
    /// The function which maps points to a noise value.
    pub func: &'a FunctionNoiseFunction
}

impl<'a> FunctionNoise<'a> {
    /// Create a new FunctionNoise with the given function.
    #[inline]
    pub fn new(func: &'a FunctionNoiseFunction)
        -> FunctionNoise<'a> { FunctionNoise { func: func } }

}

impl<'a> Clone for FunctionNoise<'a> {
	fn clone(&self) -> FunctionNoise<'a> {
		FunctionNoise { func: self.func }
	}
}

impl<'a> NoiseModule for FunctionNoise<'a> {
    #[inline]
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str> {
        (*self.func)(v.x, v.y)
    }
}
