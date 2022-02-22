use serde::{Deserialize, Serialize};

use crate::renderer::{
    core::{color, color::Color, math::min, ray::Ray, vector},
    scene::{hittable::HitRecord, materials::Scatterable},
};
use rand::Rng;

#[derive(Serialize, Deserialize, Debug)]
pub struct Dielectric {
    pub ior: f32,
}

impl Scatterable for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let ref_ratio = if rec.front_face {
            1.0 / self.ior
        } else {
            self.ior
        };

        let unit_dir = vector::unit_vector(&r_in.dir);
        let cos_theta = min!(vector::dot(&(-unit_dir), &rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let dir = if ref_ratio * sin_theta > 1.0
            || Dielectric::reflectance(cos_theta, ref_ratio)
                > rand::thread_rng().gen_range(0.0..1.0)
        {
            vector::reflect(&unit_dir, &rec.normal)
        } else {
            vector::refract(&unit_dir, &rec.normal, ref_ratio)
        };

        Some((Ray::new(rec.p, dir), color::WHITE))
    }
}

impl Dielectric {
    fn reflectance(cos: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Default for Dielectric {
    fn default() -> Self {
        Self { ior: 1.0 }
    }
}
