use crate::renderer::core::color::Color;
use crate::renderer::core::ray::Ray;
use crate::renderer::scene::hittable::HitRecord;

/// The trait all material types must implement.
trait Material {
    /// The scattering calculation for the material. Given the hit record, it
    /// should calculate how the ray scatters and the attenuation given to the color
    /// that produces.
    fn scatter(r_in: &Ray, rec: &HitRecord, attenuation: &Color, scattered: &Ray) -> bool;
}
