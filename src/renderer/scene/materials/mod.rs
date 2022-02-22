pub mod dielectric;
pub mod lambert;
pub mod metal;

use crate::renderer::core::color::Color;
use crate::renderer::core::ray::Ray;
use crate::renderer::scene::hittable::HitRecord;
use dielectric::Dielectric;
use lambert::Lambert;
use metal::Metal;
use serde::{Deserialize, Serialize};

/// The trait all material types must implement.
pub trait Scatterable {
    /// The scattering calculation for the material. Given the hit record, it
    /// should calculate how the ray scatters and the attenuation given to the color
    /// that produces.
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Material {
    Lambert(Lambert),
    Metal(Metal),
    Dielectric(Dielectric),
    Default,
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Material::Lambert(x) => x.scatter(r_in, rec),
            Material::Metal(x) => x.scatter(r_in, rec),
            Material::Dielectric(x) => x.scatter(r_in, rec),
            _ => {
                let default = Lambert {
                    albedo: Color::new(0.5, 0.5, 0.5),
                };
                default.scatter(r_in, rec)
            }
        }
    }
}
