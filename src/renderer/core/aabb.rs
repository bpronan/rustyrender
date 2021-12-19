use crate::renderer::core::vector::Point3;
use crate::renderer::core::ray::Ray;

use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct AABB {
    box_min: Point3,
    box_max: Point3,
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

impl AABB {

    pub fn new(box_min: Point3, box_max: Point3) -> AABB {
        AABB {
            box_min: box_min,
            box_max: box_max,
        }
    }

    pub fn expand(&mut self, other: AABB) {
        self.box_min.x = min!(other.box_min.x, self.box_min.x);
        self.box_min.y = min!(other.box_min.x, self.box_min.y);
        self.box_min.z = min!(other.box_min.x, self.box_min.z);
        self.box_max.x = max!(other.box_max.x, self.box_max.x);
        self.box_max.y = max!(other.box_max.y, self.box_max.y);
        self.box_max.z = max!(other.box_max.z, self.box_max.z);
    }

    pub fn hit(&self, r: &Ray, t: f32) -> bool {
        let tx1 = (self.box_min.x - r.orig.y)*r.invdir.y;
        let tx2 = (self.box_max.x - r.orig.y)*r.invdir.y;
      
        let mut tmin = min!(tx1, tx2);
        let mut tmax = max!(tx1, tx2);
      
        let ty1 = (self.box_min.y - r.orig.y)*r.invdir.y;
        let ty2 = (self.box_max.y - r.orig.y)*r.invdir.y;
      
        tmin = max!(tmin, min!(ty1, ty2));
        tmax = min!(tmax, max!(ty1, ty2));
      
        let tz1 = (self.box_min.z - r.orig.z)*r.invdir.z;
        let tz2 = (self.box_max.z - r.orig.z)*r.invdir.z;
      
        tmin = max!(tmin, min!(tz1, tz2));
        tmax = min!(tmax, max!(tz1, tz2));
      
        return tmax >= max!(0.0, tmin) && tmin < t;
    }
}

