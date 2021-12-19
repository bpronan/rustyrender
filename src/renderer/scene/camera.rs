use crate::renderer::core::ray::Ray;
use crate::renderer::core::vector::{ Point3, Vec3 };



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

    /// Returns a ray using the uv coordinates of the point on the film plane.
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.ll_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera() {

        let c = Camera::new(1.0, 1.0, 10.0, 100, 100);

        let r = c.get_ray(0.0, 0.0);

        let p = r.orig;
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
        assert_eq!(p.z, 0.0);
        
        let d = r.dir;
        assert_eq!(d.x, -0.5);
        assert_eq!(d.y, -0.5);
        assert_eq!(d.z, -10.0);

        let r = c.get_ray(1.0, 1.0);

        let p = r.orig;
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
        assert_eq!(p.z, 0.0);
        
        let d = r.dir;
        assert_eq!(d.x, 0.5);
        assert_eq!(d.y, 0.5);
        assert_eq!(d.z, -10.0);

    }
    
}
