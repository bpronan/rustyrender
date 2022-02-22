use serde::{Deserialize, Serialize};

use crate::renderer::{
    core::{color::Color, ray::Ray, vector},
    scene::{hittable::HitRecord, materials::Scatterable},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Scatterable for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = vector::reflect(&r_in.dir, &rec.normal);

        let scatter_dir = Ray::new(rec.p, reflected + self.fuzz * vector::random_unit_vector());

        if vector::dot(&scatter_dir.dir, &rec.normal) > 0.0 {
            return Some((scatter_dir, self.albedo));
        }
        None
    }
}

impl Default for Metal {
    fn default() -> Self {
        Self {
            albedo: Color::new(0.5, 0.5, 0.5),
            fuzz: 0.0,
        }
    }
}
