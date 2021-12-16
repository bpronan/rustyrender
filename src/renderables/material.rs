use crate::core::ray::Ray;
use crate::core::vector::Color;
use crate::renderables::world::HitRecord;


trait Material {
    fn scatter(r_in: &Ray, rec: &HitRecord, attenuation: &Color, scattered: &Ray) -> bool;
}