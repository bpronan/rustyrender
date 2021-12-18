use rand::Rng;
use crate::renderer::core::ray::Ray;
use crate::renderer::core::vector;
use crate::renderer::core::vector::{ Color, Point3, Vec3 };
use crate::renderer::scene::world::{ HittableList, Hittable, HitRecord };

use super::context::RenderContext;

#[inline]
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

pub fn ray_color_it(r: &Ray, world: &HittableList, depth: u32) -> Color {
    let mut rec = HitRecord {
        p: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        t: f32::INFINITY,
        front_face: false,
    };

    let mut color = Color::new(0.0, 0.0, 0.0);

    let mut curr_ray = *r;

    for n in 0..depth {
        if !world.hit(&curr_ray, 0.001, f32::INFINITY, &mut rec)
        {
            let unit_direction = vector::unit_vector(&curr_ray.dir);
            let t = 0.5 * (unit_direction.y() + 1.0);
            color = (Color::new(1.0, 1.0, 1.0) * (1.0 - t) 
                + Color::new(0.5, 0.7, 1.0) 
                * t) * 0.5_f32.powi((n) as i32);
            break;
        }
        let target = rec.p + rec.normal + vector::random_in_unit_sphere();
        curr_ray.orig = rec.p;
        curr_ray.dir = target - rec.p;

        rec.reset();
    }

    color
}

pub fn render_pixel(ctx_arc: &RenderContext, world: &HittableList, x: usize, y: usize) -> Color {
    let mut pixel = Color::new(0.0, 0.0, 0.0);
    let w = ctx_arc.camera.film_width as usize;
    let h = ctx_arc.camera.film_height as usize;
    let pixel_scale = 1.0 / (ctx_arc.samples as f32);

    for _n in 0..ctx_arc.samples {
        let mut rng = rand::thread_rng();
        let randx = rng.gen_range(0.0..1.0);
        let randy = rng.gen_range(0.0..1.0);
        let u = ((x as f32) + randx) / ((w - 1) as f32);
        let v = (((h - y) as f32) + randy) / ((h - 1) as f32);

        let r = ctx_arc.camera.get_ray(u, v);

        pixel += ray_color_it(&r, world, ctx_arc.max_depth);
    }

    pixel *= pixel_scale;

    pixel
}