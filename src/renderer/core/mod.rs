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

pub(crate) use debug_check;
