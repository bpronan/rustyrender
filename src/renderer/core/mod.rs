//! A collection of the core mathematical concepts and their features.

/// A library containing the representation of a RGB floating point color
pub mod color;
/// A library containing a mathematical ray with an origin and direction.
pub mod ray;
/// A library containing the representation of a 3D vector object.
pub mod vector;

/// A library containing the representation of a axis aligned bounding box
pub mod aabb;

/// This macro takes an expression as an argument and will
/// log to error and panic on debug only. This is useful for
/// precondition checks for internal APIs.
macro_rules! debug_check {
    ($expression:expr) => {
        if (!$expression) {
            error!("debug assertion failed: {}", stringify!($expression));
            debug_assert!($expression);
        }
    };
}

/// Clamp a value between min and max.
#[inline]
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

/// Clamps the pixel and converts it to linear space.
#[inline]
pub fn convert_pixel(pixel: vector::Vec3) -> (u8, u8, u8) {
    (
        (clamp(f32::sqrt(pixel.x), 0.0, 0.999) * 256.0) as u8,
        (clamp(f32::sqrt(pixel.y), 0.0, 0.999) * 256.0) as u8,
        (clamp(f32::sqrt(pixel.z), 0.0, 0.999) * 256.0) as u8,
    )
}

/// Writes a pixel to a buffer location
macro_rules! write_pixel {
    ($pixel:expr, $out:expr, $location:expr) => {{
        let (r, g, b) = convert_pixel($pixel);

        $out[$location * 3] = r;
        $out[$location * 3 + 1] = g;
        $out[$location * 3 + 2] = b;
    }};
}

pub(crate) use debug_check;
pub(crate) use write_pixel;

/// Implementation of max since std::cmp::max doesn't work for f32
macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}

/// Implementation of max since std::cmp::max doesn't work for f32
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min!($($z),*);
        if $x < y {
            $x
        } else {
            y
        }
    }}
}

pub(crate) use max;
pub(crate) use min;
