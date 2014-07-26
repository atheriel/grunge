/*
    This file is part of grunge, a coherent noise generation library.
*/

//! Primitive noise-making functions and traits.
//!
//! Usually, the higher-level implementations available in the [modules]
//! (../modules/index.html) module are more desirable than using these functions
//! on their own.

use cgmath::vector::{Vector, Vector2, Vector3, Vector4, dot};

/// NoiseModules are objects that can be asked to generate procedural noise
/// values for a given coordinate.
///
/// These are the primary interfaces provided by the library for working with
/// noise, although more primitive functions are available.
pub trait NoiseModule: Clone {
    /// Generates a noise value for the given coordinates. It is possible for
    /// this method to fail or be impossible, and in this case the Result will
    /// contain an appropriate error message.
    fn generate_2d(&self, v: Vector2<f32>) -> Result<f32, &str>;

    #[experimental]
    fn to_box(&self) -> Box<NoiseModule> {
        box self.clone() as Box<NoiseModule>
    }
}

/// The factor needed to skew x-y coordinates to coordinates on the grid of
/// simplexes in two dimensions. Approximates $\frac{\sqrt{3} - 1}{2}$.
static HAIRY_2D: f32 = 0.366025403784439;

/// The factor needed to unskew coordinates in the grid of simplexes to x-y
/// coordinates in two dimensions. Approximates $\frac{3 - \sqrt{3}}{6}$.
static SKEW_2D: f32 = 0.211324865405187;

/// Generate the coherent noise value for a point using the Simplex Noise
/// method proposed by Ken Perlin [1].
///
/// Simplex noise in 2D essentially works by overlaying the plane with a grid of
/// equilateral triangles (which are 2-simplexes) and pre-assigning a noise
/// value to the verticies of these triangles. Then, we simply find which
/// triangle the desired point is in, and interpolate from the verticies of that
/// triangle to the point.
///
/// This implementation follows the GLSL code of McEwan et al. (2012) [2], with
/// some changes based on Stefan Gustavson's Java code [3].
///
/// 1. Perlin, Ken. (2002). _Improving Noise_. ACM Transactions on Graphics
///    (Proceedings of SIGGRAPH 2002) 21(3): 681-682.
/// 2. McEwan, Ian, David Sheets, Stefan Gustavson, and Mark Richardson. (2012).
///    [_Efficient Computational Noise in GLSL_]
///    (http://dx.doi.org/10.1080/2151237X.2012.649621). Journal of Graphics
///    Tools 16(2): 85-94.
/// 3. Gustavson, Stefan. (2005). [_Simplex Noise Demystified_]
///    (http://www.itn.liu.se/~stegu/simplexnoise/simplexnoise.pdf).
///    Technical Report. Linköping University, Sweden.
pub fn snoise_2d(v: Vector2<f32>, seed: uint) -> f32 {
    // First, determine which cell of N! = 2 simplexes we are in, and where
    // in that cell we are.
    //
    // 1. Skew the input vector from Euclidian space to where it would lie on
    //    a grid of simplexes.
    // 2. Take the integer part (i.e. floor) of the value to get the
    //    coordinates of the simplex cell. That is, the corner of the cell
    //    closest to the origin.
    let i0 = Vector2::new(
        (v.x + (v.x + v.y) * HAIRY_2D).floor(),
        (v.y + (v.x + v.y) * HAIRY_2D).floor()
    );

    // 3. Work out where the point lies within the cell; that is, find the
    //    vector from the coordinates of the cell to the point.
    let x0 = v - i0 + Vector2::new(SKEW_2D, SKEW_2D).mul_s(i0.x + i0.y);

    // Now we need the location of the other two corners of the simplex. In
    // two dimensions knowing the cell means knowing two of three corners
    // automatically, but we still need the third.
    //
    // Conveniently, in two dimensions which of the simplexes in the cell the
    // point lies in is simply a matter of whether x > y or y > x. If the
    // former, then the point is in the "lower" simplex, while in the latter
    // case it is in the "upper" simplex.
    let i1: Vector2<f32> =
        if x0.x > x0.y { Vector2::unit_x() } else { Vector2::unit_y() };

    // And now we have the locations of the other two corners, which we
    // convert back to unskewed coordinates.
    let x1 = x0 - i1 + Vector2::new(SKEW_2D, SKEW_2D);
    let x2 = x0 + Vector2::new(-1.0 + 2.0 * SKEW_2D, -1.0 + 2.0 * SKEW_2D);

    let mut m = Vector3::new(
        (0.5f32 - dot(x0, x0)).max(0.0f32),
        (0.5f32 - dot(x1, x1)).max(0.0f32),
        (0.5f32 - dot(x2, x2)).max(0.0f32)
    );
    m = m * m * m;

    // If you expect to have large values for the input point, it may be a
    // good idea to mark i0 as mutable and take $i_0 = i_0 \% 289$:
    // i0 = Vector2::new(i0.x % 289, i0.y % 289);
    let fseed = seed as f32;

    let p = ((Vector3::new(i0.y, i0.y + i1.y, i0.y + 1.0).permutation_hash()
            + Vector3::new(i0.x, i0.x + i1.x, i0.x + 1.0)).permutation_hash()
            + Vector3::new(fseed, fseed, fseed))
            .permutation_hash();
    let h1 = Vector3::new(
        2.0 * (p.x * 0.024390243902439).fract() - 1.0,
        2.0 * (p.y * 0.024390243902439).fract() - 1.0,
        2.0 * (p.z * 0.024390243902439).fract() - 1.0
    );
    let h2 = Vector3::new(
        h1.x.abs() - 0.5, h1.y.abs() - 0.5, h1.z.abs() - 0.5
    );
    let h3 = Vector3::new(
        (h1.x + 0.5).floor(), (h1.y + 0.5).floor(), (h1.z + 0.5).floor()
    );
    let h4 = h1 - h3;

    m = m * Vector3::new(
        1.79284291400159 - 0.85373472095314 * ( h4.x * h4.x + h2.x * h2.x ),
        1.79284291400159 - 0.85373472095314 * ( h4.y * h4.y + h2.y * h2.y ),
        1.79284291400159 - 0.85373472095314 * ( h4.z * h4.z + h2.z * h2.z )
    );

    let g = Vector3::new(
        h4.x * x0.x + h2.x * x0.y,
        h4.y * x1.x + h2.y * x1.y,
        h4.z * x2.x + h2.z * x2.y
    );

    // Scale the result to within about [-1, 1]
    130.0 * dot(m, g)
}

/// For convenience, this trait is implemented by float-valued vectors in order
/// to make it simple to compute pseudo-random gradient indicies. It follows
/// the method laid out in McEwan et al. (2012) [1].
///
/// 1. McEwan, Ian, David Sheets, Stefan Gustavson, and Mark Richardson. (2012).
///    [_Efficient Computational Noise in GLSL_]
///    (http://dx.doi.org/10.1080/2151237X.2012.649621). Journal of Graphics
///    Tools 16(2): 85-94.
pub trait McEwanPermutable {
    /// Hashes the vector using the permutaion polynomial given by McEwan et
    /// al. (2012), i.e. $(34x\^2 + x) mod 289$. This map has reasonably good
    /// shuffling, but should not be construed as a reliable hash for other
    /// applications.
    fn permutation_hash(&mut self) -> Self;
}

macro_rules! mcewan_permutable_float (
    ($T:ident <$S:ident>, $($field:ident),+ ) => (
        impl McEwanPermutable for $T<$S> {
            #[inline]
            fn permutation_hash(&mut self) -> $T<$S> {
                $T::new(
                    $(((self.$field * 34.0 + 1.0) * self.$field) % 289.0),+
                )
            }
        }
    )
)

// Generate implementations for vectors of floats (these will actually get
// used in the library).
mcewan_permutable_float!(Vector2<f32>, x, y)
mcewan_permutable_float!(Vector3<f32>, x, y, z)
mcewan_permutable_float!(Vector4<f32>, x, y, z, w)
