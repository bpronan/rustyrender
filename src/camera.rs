use crate::ray::Ray;
use crate::vector::Point3;
use crate::vector::Vec3;

pub struct Camera {
    origin: Point3,
    ll_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        Camera {
            origin: Point3::new(0.0, 0.0, 0.0),
            horizontal: Vec3::new(viewport_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, viewport_height, 0.0),
            ll_corner: Vec3::new(-0.5 * viewport_width, -0.5 * viewport_height, -focal_length),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            orig: self.origin,
            dir: self.ll_corner + self.horizontal * u + self.vertical * v - self.origin,
        }
    }
}
