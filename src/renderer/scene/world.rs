use crate::renderer::core::aabb::AABB;
use crate::renderer::core::color;
use crate::renderer::core::color::Color;
use crate::renderer::core::ray::Ray;
use crate::renderer::core::vector;
use crate::renderer::core::vector::Point3;

use crate::renderer::scene::hittable::{Hittable, HitRecord};


/// A data structure representing a region of the scene. This can
/// be the whole scene or a self-contained portion of the scene.
/// 
/// This will be used to implement k-d trees to improve the 
/// performance further.
/// 
// REVIEW: Here's where we would write the custom serde juice to 
// marshall this region/world struct
pub struct Region {
    pub objects: Vec<Box<dyn Hittable + Sync>>,
    bounding_box: AABB,
    bg_color: Color,
}


impl Region {

    /// Create a new empty region with the specified background color.
    /// The background color will be used when the ray doesn't intersect
    /// anything.
    pub fn new(bg_color: Color) -> Region {
        Region { 
            objects: Vec::new(),
            bounding_box: AABB::new(                
                Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
                Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
            ),
            bg_color: bg_color,
        }
    }

    /// Add a new object to the region. This will update the list of objects
    /// and change the bounding box to include
    pub fn push(&mut self, obj: Box<dyn Hittable + Sync>) {
        // update bounding_box
        self.bounding_box.expand(obj.bounds());
        self.objects.push(obj);
    }

    /// Gets the color for a ray that hasn't hit any objects.
    /// This would likely be a lookup into an HDRI image
    /// or cube map as the engine gets improved.
    /// 
    /// This doesn't really belong here. It should be a property
    /// of the global world only, not every logical region.
    pub fn background_color(&self, r: &Ray) -> Color {
        let unit_direction = vector::unit_vector(&r.dir);
        let t = 0.5 * (unit_direction.y + 1.0);
        color::lerp(color::WHITE, self.bg_color, t)
    }
}

/// The region implements the hittable trait as well. 
impl Hittable for Region {

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {

        // if we don't hit the world bounding box, return right away
        if !self.bounding_box.hit(r, t_max) {
            return false;
        }

        // otherwise, loop through all the objects contained within.
        let mut hit_temp = HitRecord::new();
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
        // the bounding box has been updated on insertion of the objects, so
        // just return it.
        self.bounding_box
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::renderer::scene::hittable::Hittable;
    use std::cell::RefCell;
    use crate::renderer::core::vector::Vec3;

    struct MockObject
    {
        bounds: AABB,
        should_hit: bool,
    }

    impl Hittable for MockObject {
        fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool
        {
            self.should_hit
        }

        fn bounds(&self) -> AABB {
            self.bounds
        }

    }

    #[test]
    fn test_region() {

        let mut mock_obj = MockObject { 
            bounds: AABB::new(Point3::new(13.0, 0.11, 12.0), Point3::new(17.0, 0.23, 16.0)),
            should_hit: true,
        };

        let mut r = Region::new(Color::new(0.1, 0.1, 0.1));

        r.push(Box::new(mock_obj));

        assert_eq!(r.bounding_box.box_min.x, 13.0);
        assert_eq!(r.bounding_box.box_min.y, 0.11);
        assert_eq!(r.bounding_box.box_min.z, 12.0);
        assert_eq!(r.bounding_box.box_max.x, 17.0);
        assert_eq!(r.bounding_box.box_max.y, 0.23);
        assert_eq!(r.bounding_box.box_max.z, 16.0);

        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let mut rec = HitRecord::new();
        assert!(!r.hit(&ray, 0.001, f32::INFINITY, &mut rec));

        // TODO: figure out how to properly mock this object with RefCell
        // or use a mock object.
    }
    
}