use crate::renderer::core::aabb::AABB;
use crate::renderer::core::ray::Ray;
use crate::renderer::core::vector;
use crate::renderer::core::vector::{Point3, Vec3};

use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = vector::dot(&r.dir, outward_normal) < 0.0;

        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -(*outward_normal);
        }
    }

    pub fn reset(&mut self) {
        self.p = Point3::new(0.0, 0.0, 0.0);
        self.normal = Vec3::new(0.0, 0.0, 0.0);
        self.t = f32::INFINITY;
        self.front_face = false;
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;

    fn bounds(&self) -> AABB;
}
