use crate::renderer::core::aabb::AABB;
use crate::renderer::core::color;
use crate::renderer::core::color::Color;
use crate::renderer::core::ray::Ray;
use crate::renderer::core::vector;
use crate::renderer::core::vector::{ Point3, Vec3 };

use crate::renderer::scene::hittable::{Hittable, HitRecord};


// REVIEW: Here's where we would write the custom serde juice to 
// marshall this region/world struct
pub struct Region {
    pub objects: Vec<Box<dyn Hittable + Sync>>,
    bounding_box: AABB,
    bg_color: Color,
}


impl Region {

    pub fn new() -> Region {
        Region { 
            objects: Vec::new(),
            bounding_box: AABB::new(                
                Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
                Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
            ),
            bg_color: Color::new(0.5, 0.7, 1.0),
        }
    }

    pub fn push(&mut self, obj: Box<dyn Hittable + Sync>) {
        // update bounding_box
        self.bounding_box.expand(obj.bounds());
        self.objects.push(obj);
    }

    /// Gets the color for a ray that hasn't hit any objects.
    /// 
    /// This doesn't really belong here. It should be a property
    /// of the global world.
    /// 
    /// This would likely be a lookup into an HDRI image
    /// or cube map.
    pub fn background_color(&self, r: &Ray) -> Color {
        let unit_direction = vector::unit_vector(&r.dir);
        let t = 0.5 * (unit_direction.y + 1.0);
        color::lerp(color::WHITE, self.bg_color, t)
    }
}

impl Hittable for Region {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {

        // if we don't hit the world bounding box, return right away
        if !self.bounding_box.hit(r, t_max) {
            return false;
        }

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

    fn bounds(&self) -> AABB {
        self.bounding_box
    }
}
