use crate::renderer::core::ray::Ray;
use crate::renderer::core::vector::Point3;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct AABB {
    pub box_min: Point3,
    pub box_max: Point3,
}

/// Implementation of max since std::cmp::max doesn't work for f32
macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}

/// Implementation of max since std::cmp::max doesn't work for f32
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min!($($z),*);
        if $x < y {
            $x
        } else {
            y
        }
    }}
}

/// An implementation of an axis-aligned bounding box. This
/// can be used for quickly checking whether an object or region
/// has any intersections before trying to find out hit information.
impl AABB {
    /// Creates an AABB from the two points specified.
    ///
    /// Params:
    /// * `box_min` - The lower left front most point of the box.
    /// * `box_max` - the upper right back most point of the box.
    pub fn new(box_min: Point3, box_max: Point3) -> AABB {
        AABB {
            box_min: box_min,
            box_max: box_max,
        }
    }

    /// Expands the bounding box to include other.
    pub fn expand(&mut self, other: AABB) {
        self.box_min.x = min!(other.box_min.x, self.box_min.x);
        self.box_min.y = min!(other.box_min.y, self.box_min.y);
        self.box_min.z = min!(other.box_min.z, self.box_min.z);
        self.box_max.x = max!(other.box_max.x, self.box_max.x);
        self.box_max.y = max!(other.box_max.y, self.box_max.y);
        self.box_max.z = max!(other.box_max.z, self.box_max.z);
    }

    /// Returns whether the ray will intersect the bounding box.
    pub fn hit(&self, r: &Ray, t: f32) -> bool {
        let tx1 = (self.box_min.x - r.orig.y) * r.invdir.y;
        let tx2 = (self.box_max.x - r.orig.y) * r.invdir.y;

        let mut tmin = min!(tx1, tx2);
        let mut tmax = max!(tx1, tx2);

        let ty1 = (self.box_min.y - r.orig.y) * r.invdir.y;
        let ty2 = (self.box_max.y - r.orig.y) * r.invdir.y;

        tmin = max!(tmin, min!(ty1, ty2));
        tmax = min!(tmax, max!(ty1, ty2));

        let tz1 = (self.box_min.z - r.orig.z) * r.invdir.z;
        let tz2 = (self.box_max.z - r.orig.z) * r.invdir.z;

        tmin = max!(tmin, min!(tz1, tz2));
        tmax = min!(tmax, max!(tz1, tz2));

        return tmax >= max!(0.0, tmin) && tmin < t;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::core::ray::Ray;
    use crate::renderer::core::vector::Vec3;

    #[test]
    fn test_boxes() {
        let mut b1 = AABB::new(Point3::new(1.0, 2.0, 3.0), Point3::new(6.0, 6.0, 6.0));
        let b2 = AABB::new(Point3::new(-1.0, -2.0, -3.0), Point3::new(4.0, 10.0, 4.0));

        b1.expand(b2);

        assert_eq!(b1.box_min.x, -1.0);
        assert_eq!(b1.box_min.y, -2.0);
        assert_eq!(b1.box_min.z, -3.0);
        assert_eq!(b1.box_max.x, 6.0);
        assert_eq!(b1.box_max.y, 10.0);
        assert_eq!(b1.box_max.z, 6.0);

        let b3 = AABB::new(Point3::new(1.0, -4.0, 3.0), Point3::new(10.0, 4.0, 10.0));

        b1.expand(b3);

        assert_eq!(b1.box_min.x, -1.0);
        assert_eq!(b1.box_min.y, -4.0);
        assert_eq!(b1.box_min.z, -3.0);
        assert_eq!(b1.box_max.x, 10.0);
        assert_eq!(b1.box_max.y, 10.0);
        assert_eq!(b1.box_max.z, 10.0);

        let b4 = AABB::new(
            Point3::new(-10.0, -4.0, -10.0),
            Point3::new(10.0, 10.0, 10.0),
        );

        b1.expand(b4);

        assert_eq!(b1.box_min.x, -10.0);
        assert_eq!(b1.box_min.y, -4.0);
        assert_eq!(b1.box_min.z, -10.0);
        assert_eq!(b1.box_max.x, 10.0);
        assert_eq!(b1.box_max.y, 10.0);
        assert_eq!(b1.box_max.z, 10.0);

        let b1 = AABB::new(Point3::new(-1.0, -1.0, -3.0), Point3::new(1.0, 1.0, -4.0));
        let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        assert!(b1.hit(&r, f32::INFINITY));
        let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 10.0, -1.0));
        assert!(!b1.hit(&r, f32::INFINITY));
    }
}
