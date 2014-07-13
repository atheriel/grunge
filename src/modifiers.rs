/*
    This file is part of grunge, a coherent noise generation library.
*/

//! Types for generating noise by taking a source noise generator and modifying
//! its output in some way.

use cgmath::vector::Vector2;

use primitives::NoiseModule;

/// This trait provides a nice way of turning unmodified noise into modified
/// noise. All NoiseModule implementations also implement this trait, so that
/// you can usually call `noise.clamp(0.0, 1.0)` instead of instantiating the
/// ClampedNoise type directly.
pub trait Modifiable : NoiseModule {
    /// Modifies a source noise module by bounding its output between a `min`
    ///  and `max` value.
    fn clamp<'a>(&'a self, min: f32, max: f32) -> ClampedNoise<'a> {
        ClampedNoise { source: self, min: min, max: max }
    }

    /// Modifies a source noise module by multiplying its output by a constant
    /// `scale` and applying a constant `bias` shift up or downwards.
    fn scalebias<'a>(&'a self, scale: f32, bias: f32) -> ScaledBiasedNoise<'a> {
        ScaledBiasedNoise { source: self, scale: scale, bias: bias }
    }

    /// Modifies a source noise module by translating its input by a constant
    /// vector.
    fn translate<'a>(&'a self, translation: Vector2<f32>)
        -> TranslatedNoise<'a> {
        TranslatedNoise { source: self, translation: translation }
    }
}

/// Modifies a source noise module by bounding its output between a `min` and
/// `max` value.
///
/// ## Example
/// ClampedNoise can be created one of three ways: with `new()`, which uses the
/// default min and max values of `-1.0` and `1.0`, respectively; with a struct
/// literal, or with the `clamp()` method on a source module itself.
///
/// ```rust
/// use grunge::modules::*;
///
/// let source = PinkNoise::new(26);
/// let first_clamp = ClampedNoise::new(source);
/// let other_clamp = ClampedNoise { source: source, min: -0.5, max: 0.5 };
/// let final_clamp = source.clamp(-0.5, 0.5);
/// assert_eq!(other_clamp, final_clamp);
/// ```
pub struct ClampedNoise<'a> {
    /// The source module.
    pub source: &'a NoiseModule,

    /// The absolute lower bound for the noise output.
    pub min: f32,

    /// The absolute upper bound for the noise output.
    pub max: f32,
}

impl<'a> ClampedNoise<'a> {
    /// Creates a new ClampedNoise with a default min and max value of `-1.0`
    /// and `1.0`, respectively.
    pub fn new(source: &'a NoiseModule) -> ClampedNoise<'a> {
        ClampedNoise { source: source, min: -1.0, max: 1.0 }
    }
}

impl<'a> NoiseModule for ClampedNoise<'a> {
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str> {
        match self.source.generate_2d(v) {
            Ok(val) => if val > self.max { Ok(self.max) }
                       else if val < self.min { Ok(self.min) }
                       else { Ok(val) },
            err => err
        }
    }
}

impl<'a> Modifiable for ClampedNoise<'a> {}

/// Modifies a source noise module by multiplying its output by a constant and
/// applying a constant shift up or downwards.
pub struct ScaledBiasedNoise<'a> {
    /// The source module.
    pub source: &'a NoiseModule,

    /// The linear scaling to apply to noise output.
    pub scale: f32,

    /// The linear transformation to apply to noise output.
    pub bias: f32,
}

impl<'a> ScaledBiasedNoise<'a> {
    /// Creates a new ScaledBiasedNoise with a default scale and bias values of
    /// `1.0` and `0.0`, respectively (i.e. no change in output).
    pub fn new(source: &'a NoiseModule) -> ScaledBiasedNoise<'a> {
        ScaledBiasedNoise { source: source, scale: 1.0, bias: 0.0 }
    }
}

impl<'a> NoiseModule for ScaledBiasedNoise<'a> {
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str> {
        match self.source.generate_2d(v) {
            Ok(val) => Ok(val * self.scale + self.bias),
            err => err
        }
    }
}

impl<'a> Modifiable for ScaledBiasedNoise<'a> {}

/// Modifies a source noise module by multiplying its input by a constant vector
/// shift.
pub struct TranslatedNoise<'a> {
    /// The source module.
    pub source: &'a NoiseModule,

    /// The linear transformation to apply to input coordinates.
    pub translation: Vector2<f32>,
}

impl<'a> TranslatedNoise<'a> {
    /// Creates a new TranslatedNoise with the given source and translation.
    pub fn new(source: &'a NoiseModule, translation: Vector2<f32>)
        -> TranslatedNoise<'a> {
        TranslatedNoise { source: source, translation: translation }
    }
}

impl<'a> NoiseModule for TranslatedNoise<'a> {
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str> {
        self.source.generate_2d(v + self.translation)
    }
}

impl<'a> Modifiable for TranslatedNoise<'a> {}
