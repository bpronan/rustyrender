use super::vector::Vec3;

/// REVIEW: if I were sticking with this math library long term, I would
/// revisit this. It's imprecise to provide xyz on a color instead of
/// rgb.
pub type Color = Vec3;

/// Constant definition of the ray tracer's white point.
pub const WHITE: Color = Color {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};
/// Constant definition of the ray tracer's black point.
pub const BLACK: Color = Color {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

/// Linear interpolation for a color value.
#[inline]
pub fn lerp(from: Color, to: Color, t: f32) -> Color {
    (1.0 - t) * from + t * to
}

/// Default trait implmentation for a color. Returns white.
impl Default for Color {
    fn default() -> Self {
        WHITE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_equal(a: f32, b: f32, decimal_places: u8) -> bool {
        let factor = 10.0f32.powi(decimal_places as i32);
        let a = (a * factor).trunc();
        let b = (b * factor).trunc();
        a == b
    }

    #[test]
    fn test_lerp() {
        assert_eq!(WHITE.x, 1.0);
        assert_eq!(WHITE.y, 1.0);
        assert_eq!(WHITE.z, 1.0);

        assert_eq!(BLACK.x, 0.0);
        assert_eq!(BLACK.y, 0.0);
        assert_eq!(BLACK.z, 0.0);

        let c1 = lerp(WHITE, BLACK, 0.5);
        assert!(approx_equal(c1.x, 0.5, 4));
        assert!(approx_equal(c1.y, 0.5, 4));
        assert!(approx_equal(c1.z, 0.5, 4));

        let c1 = lerp(Color::new(0.1, 0.2, 0.3), Color::new(0.2, 0.4, 0.6), 0.5);

        assert!(approx_equal(c1.x, 0.15, 4));
        assert!(approx_equal(c1.y, 0.3, 4));
        assert!(approx_equal(c1.z, 0.45, 4));

        let c1 = lerp(Color::new(0.1, 0.2, 0.4), Color::new(0.2, 0.4, 0.8), 0.25);

        assert!(approx_equal(c1.x, 0.125, 4));
        assert!(approx_equal(c1.y, 0.25, 4));
        assert!(approx_equal(c1.z, 0.5, 4));
    }
}
