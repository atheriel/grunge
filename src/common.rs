/*
    This file is part of grunge, a coherent noise generation library.
*/

use cgmath::vector::Vector2;

/// NoiseModules are objects that can be asked to generate procedural noise
/// values for a given coordinate.
///
/// These are the primary interfaces provided by the library for working with
/// noise, although more primitive functions are available.
pub trait NoiseModule {
    /// Generates a noise value for the given coordinates. It is possible for
    /// this method to fail or be impossible, and in this case the Result will
    /// contain an appropriate error message.
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str>;

    /// Generates a noise value for the given coordinates. This method should
    /// only be usefule when a mutable version of self is required (i.e. when
    /// using closures).
    fn mut_generate_2d(&mut self, v: Vector2<f32>) -> Result<f32, &str> {
        self.generate_2d(v)
    }
}
