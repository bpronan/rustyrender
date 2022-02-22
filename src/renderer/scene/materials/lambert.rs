use serde::{Deserialize, Serialize};

use crate::renderer::{
    core::{color::Color, ray::Ray, vector},
    scene::{hittable::HitRecord, materials::Scatterable},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Lambert {
    pub albedo: Color,
}

impl Scatterable for Lambert {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_dir = rec.normal + vector::random_unit_vector();

        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }

        Some((Ray::new(rec.p, scatter_dir), self.albedo))
    }
}

impl Default for Lambert {
    fn default() -> Self {
        Self {
            albedo: Color::new(0.5, 0.5, 0.5),
        }
    }
}
