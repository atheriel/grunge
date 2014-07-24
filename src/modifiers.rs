/*
    This file is part of grunge, a coherent noise generation library.
*/

//! Types for generating noise by taking a source noise generator and modifying
//! its output in some way.
//!
//! The documentation for [Modifiable](trait.Modifiable.html) provides some more
//! detail on their use.

use cgmath::angle::rad;
use cgmath::vector::Vector2;
use cgmath::rotation::{Rotation, Rotation2, Basis2};

use primitives::NoiseModule;

// Dirty little hacks for dealing with boxes and trait type-erasure
fn clone<T: Clone>(t: &T) -> T { t.clone() }

impl Clone for Box<NoiseModule> {
    fn clone(&self) -> Box<NoiseModule> { self.to_box() }
}

/// This trait provides a nice way of turning unmodified noise into modified
/// noise. All NoiseModule implementations also implement this trait, so that
/// you can usually call `noise.clamp(0.0, 1.0)` instead of instantiating the
/// ClampedNoise type directly.
///
/// ## Example
///
/// The trait Modifiable provides a way to conveniently modify other NoiseModule
/// sources, and its methods may be chained. The following is a common way to
/// change the range of PinkNoise from [-1, 1] to [0, 1]:
///
/// ```rust
/// extern crate grunge;
///
/// use grunge::vectors::Vector2;
/// use grunge::modules::{NoiseModule, Modifiable, PinkNoise};
///
/// fn main() {
///     let noise = PinkNoise::new(37).scalebias(0.5, 0.5).clamp(0.0, 1.0);
///     println!("{}", noise.generate_2d(Vector2::new(1.0, -1.0)));
/// }
/// ```
pub trait Modifiable : NoiseModule {
    /// Modifies a source noise module by bounding its output between a `min`
    ///  and `max` value.
    fn clamp(&self, min: f32, max: f32) -> ClampedNoise {
        ClampedNoise { source: self.to_box(), min: min, max: max }
    }

    /// Modifies a source noise module by multiplying its output by a constant
    /// `scale` and applying a constant `bias` shift up or downwards.
    fn scalebias(&self, scale: f32, bias: f32) -> ScaledBiasedNoise {
        ScaledBiasedNoise { source: self.to_box(), scale: scale, bias: bias }
    }

    /// Modifies a source noise module by translating its input by a constant
    /// vector.
    fn translate(&self, translation: Vector2<f32>) -> TranslatedNoise {
        TranslatedNoise { source: self.to_box(), translation: translation }
    }

    /// Modifies a source noise module by rotating its input by an angle.
    // Note that the angle is assumed to be in radians.
    fn rotate(&self, rotation: f32) -> RotatedNoise {
        RotatedNoise {
            source: self.to_box(),
            rotation: Rotation2::from_angle(rad(rotation))
        }
    }
}

/// Modifies a source noise module by bounding its output between a `min` and
/// `max` value.
///
/// ## Example
///
/// ClampedNoise can be created one of three ways: with `new()`, which uses the
/// default min and max values of `-1.0` and `1.0`, respectively; with a struct
/// literal, or with the `clamp()` method on a source module itself. Note that
/// when using the struct literal, you will need to convert the source module to
/// a boxed representation using NoiseModule's `to_box()` method.
///
/// ```rust
/// extern crate grunge;
///
/// use grunge::vectors::Vector2;
/// use grunge::modules::{NoiseModule, Modifiable, PinkNoise, ClampedNoise};
///
/// fn main() {
///     let source = PinkNoise::new(26);
///     let first_clamp = ClampedNoise::new(&source);
///     let other_clamp = ClampedNoise {
///         source: source.to_box(), min: -0.5, max: 0.5
///     };
///     let final_clamp = source.clamp(-0.5, 0.5);
///     assert_eq!(other_clamp.generate_2d(Vector2::new(1.0, -1.0)),
///                final_clamp.generate_2d(Vector2::new(1.0, -1.0)));
/// }
/// ```
pub struct ClampedNoise {
    /// The source module.
    pub source: Box<NoiseModule>,

    /// The absolute lower bound for the noise output.
    pub min: f32,

    /// The absolute upper bound for the noise output.
    pub max: f32,
}

impl ClampedNoise {
    /// Creates a new ClampedNoise with a default min and max value of `-1.0`
    /// and `1.0`, respectively.
    pub fn new(source: &NoiseModule) -> ClampedNoise {
        ClampedNoise { source: source.to_box(), min: -1.0, max: 1.0 }
    }
}

impl Clone for ClampedNoise {
    fn clone(&self) -> ClampedNoise {
        ClampedNoise {
            source: clone(&self.source),
            min: self.min.clone(), max: self.max.clone()
        }
    }
}

impl NoiseModule for ClampedNoise {
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str> {
        match self.source.generate_2d(v) {
            Ok(val) => if val > self.max { Ok(self.max) }
                       else if val < self.min { Ok(self.min) }
                       else { Ok(val) },
            err => err
        }
    }
}

impl Modifiable for ClampedNoise {}

/// Modifies a source noise module by multiplying its output by a constant and
/// applying a constant shift up or downwards.
pub struct ScaledBiasedNoise {
    /// The source module.
    pub source: Box<NoiseModule>,

    /// The linear scaling to apply to noise output.
    pub scale: f32,

    /// The linear transformation to apply to noise output.
    pub bias: f32,
}

impl ScaledBiasedNoise {
    /// Creates a new ScaledBiasedNoise with a default scale and bias values of
    /// `1.0` and `0.0`, respectively (i.e. no change in output).
    pub fn new(source: &NoiseModule) -> ScaledBiasedNoise {
        ScaledBiasedNoise { source: source.to_box(), scale: 1.0, bias: 0.0 }
    }
}

impl Clone for ScaledBiasedNoise {
    fn clone(&self) -> ScaledBiasedNoise {
        ScaledBiasedNoise {
            source: clone(&self.source),
            scale: self.scale.clone(), bias: self.bias.clone()
        }
    }
}

impl NoiseModule for ScaledBiasedNoise {
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str> {
        match self.source.generate_2d(v) {
            Ok(val) => Ok(val * self.scale + self.bias),
            err => err
        }
    }
}

impl Modifiable for ScaledBiasedNoise {}

/// Modifies a source noise module by multiplying its input by a constant vector
/// shift.
pub struct TranslatedNoise {
    /// The source module.
    pub source: Box<NoiseModule>,

    /// The linear transformation to apply to input coordinates.
    pub translation: Vector2<f32>,
}

impl TranslatedNoise {
    /// Creates a new TranslatedNoise with the given source and translation.
    pub fn new(source: &NoiseModule, translation: Vector2<f32>)
        -> TranslatedNoise {
        TranslatedNoise { source: source.to_box(), translation: translation }
    }
}

impl Clone for TranslatedNoise {
    fn clone(&self) -> TranslatedNoise {
        TranslatedNoise {
            source: clone(&self.source), translation: self.translation.clone()
        }
    }
}

impl NoiseModule for TranslatedNoise {
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str> {
        self.source.generate_2d(v + self.translation)
    }
}

impl Modifiable for TranslatedNoise {}

/// Modifies a source noise module by rotating its input by some angle.
#[experimental]
pub struct RotatedNoise {
    /// The source module.
    pub source: Box<NoiseModule>,

    /// The rotation to apply to input coordinates.
    rotation: Basis2<f32>,
}

impl RotatedNoise {
    /// Creates a new RotatedNoise with the given source and rotation.
    pub fn new(source: &NoiseModule, rotation: f32)
        -> RotatedNoise {
        RotatedNoise {
            source: source.to_box(),
            rotation: Rotation2::from_angle(rad(rotation))
        }
    }
}

impl Clone for RotatedNoise {
    fn clone(&self) -> RotatedNoise {
        RotatedNoise {
            source: clone(&self.source), rotation: self.rotation.clone()
        }
    }
}

impl NoiseModule for RotatedNoise {
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str> {
        self.source.generate_2d(self.rotation.rotate_vector(&v))
    }
}

impl Modifiable for RotatedNoise {}

/// Functions applicable for passing to ModifierNoise.
pub type ModifierNoiseFunction = fn(x: f32, y: f32, out: f32) -> Result<f32, &str>;

/// ModifierNoise allows the use of an arbitrary function to modify noise.
#[experimental]
pub struct ModifierNoise<'a> {
    /// The source module.
    pub source: Box<NoiseModule>,

    /// The function which maps points and initial output to a noise value.
    pub func: &'a ModifierNoiseFunction
}

impl<'a> ModifierNoise<'a> {
    /// Create a new ModifierNoise with the given function.
    #[inline]
    pub fn new(source: &NoiseModule, func: &'a ModifierNoiseFunction)
        -> ModifierNoise<'a> {
            ModifierNoise { source: source.to_box(), func: func }
    }
}

impl<'a> Clone for ModifierNoise<'a> {
    fn clone(&self) -> ModifierNoise<'a> {
        ModifierNoise { source: clone(&self.source), func: self.func }
    }
}

impl<'a> NoiseModule for ModifierNoise<'a> {
    #[inline]
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str> {
        match self.source.generate_2d(v) {
            Ok(output) => (*self.func)(v.x, v.y, output),
            err => err
        }
    }
}

impl<'a> Modifiable for ModifierNoise<'a> {}
