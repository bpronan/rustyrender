use std::f32;
use std::ops;

use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    e: (f32, f32, f32),
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: (x, y, z) }
    }

    pub fn x(&self) -> f32 {
        self.e.0
    }

    pub fn y(&self) -> f32 {
        self.e.1
    }

    pub fn z(&self) -> f32 {
        self.e.2
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.e.0 * self.e.0 + self.e.1 * self.e.1 + self.e.2 * self.e.2
    }

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

pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
    u.e.0 * v.e.0 + u.e.1 * v.e.1 + u.e.2 * v.e.2
}


pub fn unit_vector(v: &Vec3) -> Vec3 {
    let invlen = 1.0 / v.length();
    Vec3::new(v.x() * invlen, v.y() * invlen, v.z() * invlen)
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut point = Vec3::random_range(-1.0, 1.0);
    let mag = point.length();
    let d = rand::thread_rng().gen_range(0.0..(1.0 / mag));
    point /= d;

    point
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            e: (-self.e.0, -self.e.1, -self.e.2),
        }
    }
}

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

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            e: (self.e.0 * other, self.e.1 * other, self.e.2 * other),
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self {
            e: (self.e.0 / other, self.e.1 / other, self.e.2 / other),
        }
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, _other: f32) {
        self.e.0 *= _other;
        self.e.1 *= _other;
        self.e.2 *= _other;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, _other: f32) {
        self.e.0 /= _other;
        self.e.1 /= _other;
        self.e.2 /= _other;
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

    let p: Point3 = Point3::new(1.2, 2.2, 3.2);
    assert_eq!(1.2, p.x());
    assert_eq!(2.2, p.y());
    assert_eq!(3.2, p.z());

    let v1 = Vec3::new(12.0, 15.0, 20.0);
    let v2 = Vec3::new(-12.0, -15.0, -20.0);
    let dot = dot(&v1, &v2);
    assert_eq!(-769.0, dot);

}
