use crate::renderer::core::color;
use crate::renderer::core::color::Color;
use crate::renderer::core::ray::Ray;
use crate::renderer::core::vector;
use crate::renderer::scene::hittable::Hittable;
use crate::renderer::scene::world::Region;
use rand::Rng;

use super::context::RenderContext;

/// A utility function to loop through a random set of samples around the given ray
/// and average out the rendered colors for each ray to get the pixel value.
pub fn render_pixel(ctx_arc: &RenderContext, world: &Region, x: usize, y: usize) -> Color {
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

        let r = ctx_arc.camera.get_ray(u, v, 1.0);

        pixel += ray_color(&r, world, ctx_arc.max_depth);
    }

    pixel *= pixel_scale;

    pixel
}

/// A utility to calculate the color of a single ray. This is the meat
/// of the raytracer algorithm. It uses an iterative loop rather than the
/// classical recursive loop in order to avoid unnecessary memory use for
/// multithreading.
#[allow(dead_code)]
fn ray_color_it(r: &Ray, world: &Region, max_depth: u32) -> Color {
    let mut color = color::BLACK;
    let mut curr_ray = *r;

    for n in 0..max_depth {
        match world.hit(&curr_ray, 0.001, f32::INFINITY) {
            Some(hit) => {
                // randomize the ray reflection to account for the micro surface plane noise
                // in a diffuse surface
                let target = hit.p + hit.normal + vector::random_unit_vector();

                curr_ray.orig = hit.p;
                curr_ray.dir = target - hit.p;
            }
            None => {
                let diffuse = 0.5_f32.powi((n) as i32);
                color = diffuse * world.background_color(&curr_ray);
                return color;
            }
        }
    }

    color
}

fn ray_color(r: &Ray, world: &Region, depth: u32) -> Color {
    if depth == 0 {
        return color::BLACK;
    }

    match world.hit(r, 0.001, f32::INFINITY) {
        Some(hit) => match hit.material.scatter(r, &hit) {
            Some((scattered, attenuation)) => attenuation * ray_color(&scattered, world, depth - 1),
            None => color::BLACK,
        },
        None => world.background_color(r),
    }
}
