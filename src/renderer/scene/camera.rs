use std::f32::consts::PI;

use serde::{Deserialize, Serialize};

use crate::renderer::core::ray::Ray;
use crate::renderer::core::vector::{self, Point3, Vec3};

/// The configurable values for the scene camera
/// pulled out as a separate object for ease of
/// serialization to a source file.
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct CameraConfig {
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
    pub vertical_fov: f32,
    pub aperture: f32,
    pub focal_distance: f32,
}

/// A simple camera and film simulation. It's responsible
/// for casting the rays through the film plane and into
/// the scene.
#[derive(Copy, Clone, Debug)]
pub struct Camera {
    origin: Point3,
    ll_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    pub film_width: u32,
    pub film_height: u32,
    lens_radius: f32,
    normal_basis: (Vec3, Vec3),
}

impl Camera {
    /// Creates a camera using the parameters supplied.
    ///
    /// Params:
    /// * `viewport_width` - A floating point value representing the logical width of the viewport.
    /// * `viewport_height` - A floating point value representing the logical height of the viewport.
    /// * `focal_length` - The focal length of the camera along the forward vector of the camera's direction.
    /// * `width` - Storing the u32 width of the image.
    /// * `height` - Storing the u32 height of the image.
    // pub fn new(
    //     viewport_height: f32,
    //     viewport_width: f32,
    //     focal_length: f32,
    //     bounds: (u32, u32),
    // ) -> Camera {
    //     Camera {
    //         origin: Point3::new(0.0, 0.0, 0.0),
    //         horizontal: Vec3::new(viewport_width, 0.0, 0.0),
    //         vertical: Vec3::new(0.0, viewport_height, 0.0),
    //         ll_corner: Vec3::new(-0.5 * viewport_width, -0.5 * viewport_height, -focal_length),
    //         film_width: bounds.0,
    //         film_height: bounds.1,
    //     }
    // }
    pub fn new(config: CameraConfig, bounds: (u32, u32)) -> Camera {
        let aspect_ratio = (bounds.0 as f32) / (bounds.1 as f32);
        let theta = config.vertical_fov * PI / 180.0;
        let h = (theta * 0.5).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = vector::unit_vector(&(config.look_from - config.look_at));
        let u = vector::unit_vector(&vector::cross(&config.up, &w));
        let v = vector::cross(&w, &u);

        Camera {
            origin: config.look_from,
            horizontal: config.focal_distance * viewport_width * u,
            vertical: config.focal_distance * viewport_height * v,
            ll_corner: config.look_from
                - config.focal_distance
                    * (viewport_width * u * 0.5 + viewport_height * v * 0.5 + w),
            film_width: bounds.0,
            film_height: bounds.1,
            lens_radius: config.aperture * 0.5,
            normal_basis: (u, v),
        }
    }

    /// Returns a ray using the uv coordinates of the point on the film plane.
    pub fn get_ray(&self, u: f32, v: f32, fuzz: f32) -> Ray {
        let rd = self.lens_radius * fuzz * vector::random_in_unit_disk();
        let offset = self.normal_basis.0 * rd.x + self.normal_basis.1 * rd.y;

        Ray::new(
            self.origin + offset,
            self.ll_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera() {
        let c = Camera::new(
            CameraConfig {
                look_from: Vec3::new(0.0, 0.0, 0.0),
                look_at: Vec3::new(0.0, 0.0, -1.0),
                up: Vec3::new(0.0, 1.0, 0.0),
                aperture: 2.0,
                focal_distance: 1.0,
                vertical_fov: 90.0,
            },
            (100, 100),
        );

        let r = c.get_ray(0.0, 0.0, 0.0);

        let p = r.orig;
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
        assert_eq!(p.z, 0.0);

        let d = r.dir;
        assert_eq!(d.x, -1.0);
        assert_eq!(d.y, -1.0);
        assert_eq!(d.z, -1.0);

        let r = c.get_ray(1.0, 1.0, 0.0);

        let p = r.orig;
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
        assert_eq!(p.z, 0.0);

        let d = r.dir;
        assert_eq!(d.x, 1.0);
        assert_eq!(d.y, 1.0);
        assert_eq!(d.z, -1.0);
    }
}
