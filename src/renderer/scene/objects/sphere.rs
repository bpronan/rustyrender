use crate::renderer::core::aabb::AABB;
use crate::renderer::core::ray::Ray;
use crate::renderer::core::vector;
use crate::renderer::core::vector::Point3;

use crate::renderer::scene::hittable::{ HitRecord, Hittable };

use serde::{Serialize, Deserialize};

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
