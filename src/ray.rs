use crate::vector;
use crate::vector::Point3;
use crate::vector::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(self, t: f32) -> Point3 {
        self.orig + self.dir * t
    }
}

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
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
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

#[test]
fn test_rays() {
    let r = Ray {
        orig: Point3::new(1.0, 2.0, 3.0),
        dir: Vec3::new(1.0, 2.0, 3.0),
    };

    let v = r.at(2.0);
    assert_eq!(3.0, v.x());
    assert_eq!(6.0, v.y());
    assert_eq!(9.0, v.z());
}
