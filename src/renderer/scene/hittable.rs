use crate::renderer::core::aabb::Aabb;
use crate::renderer::core::ray::Ray;
use crate::renderer::core::vector;
use crate::renderer::core::vector::{Point3, Vec3};

use serde::{Deserialize, Serialize};

/// A data structure keeping track of the important
/// information about where the ray hit. Tracks the
/// point in space, surface normal, and face information.
///
/// Any new scene object must populate this data structure.
/// Fields:
/// * `p` - The point of intersection.
/// * `normal` - The surface normal at the intersection.
/// * `t` - The t along the ray. Used by the algorithm to
/// ensure objects are sorted in z.
/// * `front_face` - Whether the hit was on the front face
/// or the back face of a surface.
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    /// Sets the face normal according to whether the intersection
    /// was on the front or the back face.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = vector::dot(&r.dir, outward_normal) < 0.0;

        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -(*outward_normal);
        }
    }

    /// Resets the record. It's important to reset this information between
    /// ray casting iterations.
    pub fn reset(&mut self) {
        self.p = Point3::new(0.0, 0.0, 0.0);
        self.normal = Vec3::new(0.0, 0.0, 0.0);
        self.t = f32::INFINITY;
        self.front_face = false;
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: f32::INFINITY,
            front_face: false,
        }
    }
}

/// The base trait for all renderable object types in the scene. Any new
/// scene object should implement this trait.
#[typetag::serde(tag = "type")]
pub trait Hittable: Sync {
    /// The meat of the ray tracing algorithm for each object. The object
    /// must implement this function to calculate where the ray intersects
    /// it and return back the correct information in the output rec parameter.
    ///
    /// Params:
    /// * `r` - The ray (origin, direction) to calculate the hit.
    /// * `t_min` - The minimum t-value along the ray that the object must be along. Helps
    /// prevent calculating hits for internal reflections due to floating point inaccuracy.
    /// * `t_max` - The maximum t value along the ray. Used to avoid drawing objects that are
    /// further away over closer ones.
    /// * `rec` - An output parameter for keeping the hit information.
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;

    /// Returns the axis aligned bounding box of this object.
    fn bounds(&self) -> Aabb;
}
