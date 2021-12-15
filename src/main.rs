extern crate image;

mod camera;
mod ray;
mod sphere;
mod vector;

use rand::Rng;

use camera::Camera;
use ray::HitRecord;
use ray::Hittable;
use ray::HittableList;
use ray::Ray;
use sphere::Sphere;
use vector::Color;
use vector::Point3;
use vector::Vec3;

use std::fs;

#[inline]
fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    let mut rec = HitRecord {
        p: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        t: f32::INFINITY,
        front_face: false,
    };

    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.001, f32::INFINITY, &mut rec) {
        let target = rec.p + rec.normal + vector::random_in_unit_sphere();
        return ray_color(
            &Ray {
                orig: rec.p,
                dir: target - rec.p,
            },
            world,
            depth - 1,
        ) * 0.5;
    }

    let unit_direction = vector::unit_vector(&r.dir);
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let imgx = 400;
    let imgy = ((imgx as f32) / aspect_ratio) as u32;
    let samples_per_pixel = 10;
    let pixel_scale = 1.0 / (samples_per_pixel as f32);
    let max_depth = 50;

    // World
    let mut world = HittableList {
        objects: Vec::new(),
    };
    world.objects.push(Box::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    world.objects.push(Box::new(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    // camera
    let camera = Camera::new();

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    let mut rng = rand::thread_rng();

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);

        for _n in 0..samples_per_pixel {
            let randx = rng.gen_range(0.0..1.0);
            let randy = rng.gen_range(0.0..1.0);
            let u = ((x as f32) + randx) / ((imgx - 1) as f32);
            let v = (((imgy - y) as f32) + randy) / ((imgy - 1) as f32);

            let r = camera.get_ray(u, v);

            pixel_color += ray_color(&r, &world, max_depth);
        }

        pixel_color *= pixel_scale;
        *pixel = image::Rgb([
            (clamp(f32::sqrt(pixel_color.x()), 0.0, 0.999) * 256.0) as u8,
            (clamp(f32::sqrt(pixel_color.y()), 0.0, 0.999) * 256.0) as u8,
            (clamp(f32::sqrt(pixel_color.z()), 0.0, 0.999) * 256.0) as u8,
        ]);
    }

    if !fs::metadata("output").is_ok() {
        fs::create_dir("output").unwrap();
    }

    imgbuf.save("output/render.png").unwrap();
}
