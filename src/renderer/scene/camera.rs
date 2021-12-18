use crate::renderer::core::ray::Ray;
use crate::renderer::core::vector::{ Point3, Vec3 };


#[derive(Copy, Clone, Debug)]
pub struct Camera {
    origin: Point3,
    ll_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    pub film_width: u32,
    pub film_height: u32,
}

impl Camera {
    pub fn new(
        viewport_height: f32, 
        viewport_width: f32, 
        focal_length: f32, 
        width: u32, height: u32) -> Camera {

        Camera {
            origin: Point3::new(0.0, 0.0, 0.0),
            horizontal: Vec3::new(viewport_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, viewport_height, 0.0),
            ll_corner: Vec3::new(-0.5 * viewport_width, -0.5 * viewport_height, -focal_length),
            film_width: width,
            film_height: height,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            orig: self.origin,
            dir: self.ll_corner + self.horizontal * u + self.vertical * v - self.origin,
        }
    }
}

