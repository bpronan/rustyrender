use crate::renderer::core::ray::Ray;
use crate::renderer::core::vector;
use crate::renderer::core::vector::{ Point3, Vec3 };

#[derive(Copy, Clone)]
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
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Sync>>,
}


impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut hit_temp = HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: f32::INFINITY,
            front_face: false,
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for hittable in &self.objects {
            if hittable.hit(r, t_min, closest_so_far, &mut hit_temp) {
                hit_anything = true;
                closest_so_far = hit_temp.t;
                *rec = hit_temp;
            }
        }

        hit_anything
    }
}