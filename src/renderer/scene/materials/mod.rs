use crate::renderer::core::ray::Ray;
use crate::renderer::core::{color, color::Color, min, vector};
use crate::renderer::scene::hittable::HitRecord;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Since this is likely to be a closed set of materials,
/// it's easier to implement this as part of an enum rather
/// than relying on the dynamic dispatch mess. By closed set,
/// we are likely to add only a textured surface and lighting.
/// Note: It's slightly inefficient to use this polymorphic
/// model for dielectrics and lamberts because the
/// size is the max for the metal. This is not likely to
/// be an issue with the current implementation.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Material {
    Lambert { albedo: Color },
    Metal { albedo: Color, fuzz: f32 },
    Dielectric { ior: f32 },
}

impl Material {
    /// Scatter the light according to the material properties.
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Material::Lambert { albedo } => {
                let mut scatter_dir = rec.normal + vector::random_unit_vector();

                if scatter_dir.near_zero() {
                    scatter_dir = rec.normal;
                }

                Some((Ray::new(rec.p, scatter_dir), *albedo))
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = vector::reflect(&r_in.dir, &rec.normal);

                let scatter_dir = Ray::new(rec.p, reflected + *fuzz * vector::random_unit_vector());

                if vector::dot(&scatter_dir.dir, &rec.normal) > 0.0 {
                    return Some((scatter_dir, *albedo));
                }
                None
            }
            Material::Dielectric { ior } => {
                let ref_ratio = if rec.front_face { 1.0 / ior } else { *ior };

                let unit_dir = vector::unit_vector(&r_in.dir);
                let cos_theta = min!(vector::dot(&(-unit_dir), &rec.normal), 1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let dir = if ref_ratio * sin_theta > 1.0
                    || reflectance(cos_theta, ref_ratio) > rand::thread_rng().gen_range(0.0..1.0)
                {
                    vector::reflect(&unit_dir, &rec.normal)
                } else {
                    vector::refract(&unit_dir, &rec.normal, ref_ratio)
                };

                Some((Ray::new(rec.p, dir), color::WHITE))
            }
        }
    }
}

/// A utility function for calculating the reflectance property of the
/// dielectric.
#[inline]
fn reflectance(cos: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}
