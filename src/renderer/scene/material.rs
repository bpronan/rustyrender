use crate::renderer::core::ray::Ray;
use crate::renderer::core::vector::Color;
use crate::renderer::scene::world::HitRecord;


trait Material {
    fn scatter(r_in: &Ray, rec: &HitRecord, attenuation: &Color, scattered: &Ray) -> bool;
}