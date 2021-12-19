use super::vector::{ Point3, Vec3 };

/// A simple 3D ray struct.
#[derive(Copy, Clone)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {

    /// Gets the point at t units of length along the ray. Useful
    /// for calculating ray-object intersections.
    pub fn at(self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rays() {
        let r = Ray {
            orig: Point3::new(1.0, 2.0, 3.0),
            dir: Vec3::new(1.0, 2.0, 3.0),
        };
    
        // sanity check positive
        let v = r.at(2.0);
        assert_eq!(3.0, v.x());
        assert_eq!(6.0, v.y());
        assert_eq!(9.0, v.z());

        // sanity check negative
        let v = r.at(-2.0);
        assert_eq!(-1.0, v.x());
        assert_eq!(-2.0, v.y());
        assert_eq!(-3.0, v.z());

    }
    
}
