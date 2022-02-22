use crate::renderer::core::{color::Color, vector::Vec3};
use crate::renderer::scene::{camera::CameraConfig, objects::sphere::Sphere};

use super::materials::Material;
use super::world::Region;
use rand::Rng;

/// Returns a procedurally generated region.
pub fn random_scene() -> Region {
    let mut world = Region::new(Color::new(0.5, 0.7, 0.9));

    let ground = Material::Lambert {
        albedo: Color::new(0.5, 0.5, 0.5),
    };
    world.push(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let rand = rand::thread_rng().gen_range(0.0..1.0);

            let center = Vec3::new(a as f32 + 0.9 * rand, 0.2, b as f32 + 0.9 * rand);

            if (center - Vec3::new(4.0, 0.2, 0.0)).length_squared() > 0.81 {
                let sphere_mat = if rand < 0.8 {
                    // lambert
                    let albedo = Vec3::random_range(0.0, 1.0) * Vec3::random_range(0.0, 1.0);
                    Material::Lambert { albedo }
                } else if rand < 0.95 {
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0..1.0);

                    Material::Metal { albedo, fuzz }
                } else {
                    Material::Dielectric { ior: 1.5 }
                };

                world.push(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material: sphere_mat,
                }));
            }
        }
    }

    let material = Material::Dielectric { ior: 1.5 };
    world.push(Box::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material,
    }));

    let material = Material::Lambert {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    };
    world.push(Box::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material,
    }));

    let material = Material::Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    world.push(Box::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material,
    }));

    world.camera_config = CameraConfig {
        vertical_fov: 20.0,
        look_from: Vec3::new(13.0, 2.0, 3.0),
        look_at: Vec3::new(0.0, 0.0, 0.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        aperture: 0.1,
        focal_distance: 10.0,
    };

    world
}
