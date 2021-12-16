use crate::core::ray::Ray;
use crate::core::vector;
use crate::core::vector::{ Point3, Vec3, Color };
use std::sync::Arc;

use crate::renderables::sphere::Sphere;

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
    pub objects: Vec<Arc<dyn Hittable>>,
}

#[derive(Clone, Debug)]
pub struct SphereWorld {
    pub objects: Vec<Sphere>,
}

impl Hittable for SphereWorld {
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



// TODO remove this bad boy
pub fn ray_color(r: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    let mut rec = HitRecord {
        p: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        t: f32::INFINITY,
        front_face: false,
    };

    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.001, f32::INFINITY, &mut rec) {
        let target = rec.p + rec.normal + vector::random_in_unit_sphere();
        return ray_color(
            &Ray {
                orig: rec.p,
                dir: target - rec.p,
            },
            world,
            depth - 1,
        ) * 0.5;
    }

    let unit_direction = vector::unit_vector(&r.dir);
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
}