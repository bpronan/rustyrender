use std::f32;
use std::ops;

use rand::Rng;

/// A simple 3 dimensional vector container and math struct. 
/// 
/// REVIEW: This was intended to learn rust, moving forward, 
/// it would make sense to integrate a crate like 'glam' and build
/// the necessary features off of it. The next set of features would
/// require matrices, so this is likely to go away.
#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub e: (f32, f32, f32),
}

impl Vec3 {

    /// Creates a vector from the three dimensional values.
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: (x, y, z) }
    }

    /// The value of the x dimension of the vector.
    pub fn x(&self) -> f32 {
        self.e.0
    }

    /// The value of the y dimension of the vector.
    pub fn y(&self) -> f32 {
        self.e.1
    }

    /// The value of the z dimension of the vector.
    pub fn z(&self) -> f32 {
        self.e.2
    }

    /// Calculates the length of the vector.
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Since sqrt() is a very slow function, it's often useful
    /// to compare lengths to the square of the value you are
    /// comparing it to.
    pub fn length_squared(&self) -> f32 {
        self.e.0 * self.e.0 + self.e.1 * self.e.1 + self.e.2 * self.e.2
    }

    /// Creates a random vector where the dimensional values
    /// are within a specified range.
    pub fn random_range(min: f32, max: f32) -> Vec3 {
        Vec3 {
            e: (
                rand::thread_rng().gen_range(min..max),
                rand::thread_rng().gen_range(min..max),
                rand::thread_rng().gen_range(min..max),
            ),
        }
    }
}

/// Typedefing a point to a vector. They are the same in non-homogenous
/// coordinates. Mathmatically, points can be translated, vectors can't.
/// This differentiation gives us the ability to keep the code using this
/// module future proof for when we add transformations.
pub type Point3 = Vec3;

/// Calculates the dot product of two vectors.
pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
    u.e.0 * v.e.0 + u.e.1 * v.e.1 + u.e.2 * v.e.2
}

/// Returns a normalized vector pointing in the same
/// direction as the input.
pub fn unit_vector(v: &Vec3) -> Vec3 {
    let invlen = 1.0 / v.length();
    Vec3::new(v.x() * invlen, v.y() * invlen, v.z() * invlen)
}

/// Returns a random vector on the surface of the unit sphere
pub fn random_in_unit_sphere() -> Vec3 {
    let mut point = Vec3::random_range(-1.0, 1.0);
    let mag = point.length();
    let d = rand::thread_rng().gen_range(0.0..(1.0 / mag));
    point /= d;

    point
}

/// - operator for a vector
impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            e: (-self.e.0, -self.e.1, -self.e.2),
        }
    }
}

/// a + b operator for two vectors
impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: (
                self.e.0 + other.e.0,
                self.e.1 + other.e.1,
                self.e.2 + other.e.2,
            ),
        }
    }
}

/// a - b operator for two vectors
impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            e: (
                self.e.0 - other.e.0,
                self.e.1 - other.e.1,
                self.e.2 - other.e.2,
            ),
        }
    }
}

/// a += b operator for vectors
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, _other: Self) {
        *self = Self {
            e: (
                self.e.0 + _other.e.0,
                self.e.1 + _other.e.1,
                self.e.2 + _other.e.2,
            ),
        };
    }
}

/// a* b pointwise multiplication operator for two vectors
impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            e: (
                self.e.0 * other.e.0,
                self.e.1 * other.e.1,
                self.e.2 * other.e.2,
            ),
        }
    }
}

/// f * a scalar multiplication operator
impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            e: (
                self.e.0 * other, 
                self.e.1 * other, 
                self.e.2 * other
            ),
        }
    }
}

/// a * f scalar multiplication operator (reverse of above)
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self * other.e.0, 
            self * other.e.1, 
            self * other.e.2
        )
    }
}

/// a / f32 scalar pointwise division operator
impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            e: (
                self.e.0 / other, 
                self.e.1 / other, 
                self.e.2 / other
            ),
        }
    }
}

/// f32 / a scalar pointwise division operator
impl ops::Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self / other.e.0, 
            self / other.e.1, 
            self / other.e.2
        )
    }
}


/// a *= f32 pointwise multiplication operator
impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, _other: f32) {
        self.e.0 *= _other;
        self.e.1 *= _other;
        self.e.2 *= _other;
    }
}

/// a /= f32 pointwise division operator
impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, _other: f32) {
        self.e.0 /= _other;
        self.e.1 /= _other;
        self.e.2 /= _other;
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_methods() {
        let v2 = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(v2.length_squared(), 29.0);
        assert_eq!(v2.length(), 29.0_f32.sqrt());

        // a stochastic check to make sure we don't get something outside of the range.
        for _ in 0..10000 {
            let v = Vec3::random_range(-13.0, 17.0);

            assert!(v.x() >= -13.0 && v.x() <= 17.0);
            assert!(v.y() >= -13.0 && v.y() <= 17.0);
            assert!(v.z() >= -13.0 && v.z() <= 17.0);
        }
    }

    #[test]
    fn test_overloads() {
        let mut v1 = Vec3::new(11.0, 13.0, 17.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
    
        v1 += v2;
        assert_eq!(12.0, v1.x());
        assert_eq!(15.0, v1.y());
        assert_eq!(20.0, v1.z());
    
        v1 *= 2.0;
        assert_eq!(24.0, v1.x());
        assert_eq!(30.0, v1.y());
        assert_eq!(40.0, v1.z());
    
        v1 /= 2.0;
        assert_eq!(12.0, v1.x());
        assert_eq!(15.0, v1.y());
        assert_eq!(20.0, v1.z());
    
        let v2 = -v1;
        assert_eq!(-12.0, v2.x());
        assert_eq!(-15.0, v2.y());
        assert_eq!(-20.0, v2.z());
    
        let v3 = 5.0 * v2;
        assert_eq!(-60.0, v3.x());
        assert_eq!(-75.0, v3.y());
        assert_eq!(-100.0, v3.z());
    
        let v3 = v2 * 5.0;
        assert_eq!(-60.0, v3.x());
        assert_eq!(-75.0, v3.y());
        assert_eq!(-100.0, v3.z());
    
        let v4 = v3 / 5.0;
        assert_eq!(-12.0, v4.x());
        assert_eq!(-15.0, v4.y());
        assert_eq!(-20.0, v4.z());
    
        let p: Point3 = Point3::new(1.2, 2.2, 3.2);
        assert_eq!(1.2, p.x());
        assert_eq!(2.2, p.y());
        assert_eq!(3.2, p.z());
    
        let v1 = Vec3::new(12.0, 15.0, 20.0);
        let v2 = Vec3::new(-12.0, -15.0, -20.0);
        let dot = dot(&v1, &v2);
        assert_eq!(-769.0, dot);
    
    }
    
}
