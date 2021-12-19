use crate::renderer::core::aabb::AABB;
use crate::renderer::core::ray::Ray;
use crate::renderer::core::vector;
use crate::renderer::core::vector::Point3;

use crate::renderer::scene::hittable::{ HitRecord, Hittable };

use serde::{Serialize, Deserialize};

/// An implementation of a sphere form as a hittable object.
/// Implements the Hittable trait.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = vector::dot(&oc, &r.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = f32::sqrt(discriminant);

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        true
    }

    fn bounds(&self) -> AABB {
        AABB::new(self.center - self.radius, self.center + self.radius)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::core::vector::Vec3;

    #[test]
    fn test_sphere() {
        let mut rec = HitRecord::new();

        let mut s1 = Sphere{ center: Point3::new(0.0, 0.0, -10.0), radius: 5.0 };
        let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(s1.hit(&r, 0.001, f32::INFINITY, &mut rec));

        // the normal should be pointed right back
        let n = rec.normal;
        assert_eq!(n.x, 0.0);
        assert_eq!(n.y, 0.0);
        assert_eq!(n.z, 1.0);

        // the face should be a front face since the sphere is solid
        assert!(rec.front_face);

        // ray's direction is 1.0, distance is 5 along that to center of sphere.
        assert_eq!(rec.t, 5.0);

        let p = rec.p;
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
        assert_eq!(p.z, -5.0);
        

        let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 10.0, -1.0));
        assert!(!s1.hit(&r, 0.001, f32::INFINITY, &mut rec));


    }
    
}
