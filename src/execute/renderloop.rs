use crate::core::ray::Ray;
use crate::core::vector::{ Color, Point3, Vec3 };

use super::context::RenderContext;

use crate::renderables::world::{ Hittable, HitRecord };

use crate::core::vector;

use rand::Rng;
use std::sync::Arc;

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

use std::sync::mpsc::{channel, RecvError};
use threadpool::ThreadPool;

// TODO: REmove these
use crate::renderables::world::SphereWorld;
use crate::io::FileReaderFactory;

fn render_pixel(ctx_arc: &RenderContext, world: &SphereWorld, x: usize, y: usize) -> Color {
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

        pixel += ray_color_it_mt(&r, world, ctx_arc.max_depth);
    }

    pixel *= pixel_scale;

    pixel
}

pub fn renderloop_concurrent(context: &RenderContext, world: &SphereWorld) -> Vec<u8> {
    let ctx_arc = Arc::new(context);

    let w = ctx_arc.camera.film_width as usize;
    let h = ctx_arc.camera.film_height as usize;

    let wrld_arc = Arc::new(world);

    let mut img_buf = vec![vec![0_u8; 3]; (w * h) as usize];

    crossbeam::scope(|scope| {
        for (i, e) in img_buf.iter_mut().enumerate() {
            let world = wrld_arc.clone();
            let ctx = ctx_arc.clone();
            scope.spawn(move |_| {
                let x = i % w;
                let y = i / w;
                let pixel = render_pixel(&ctx, &world, x, y);
                e[0] = (clamp(f32::sqrt(pixel.x()), 0.0, 0.999) * 256.0) as u8;
                e[1] = (clamp(f32::sqrt(pixel.y()), 0.0, 0.999) * 256.0) as u8;
                e[2] = (clamp(f32::sqrt(pixel.z()), 0.0, 0.999) * 256.0) as u8;
            });
        }
    }).expect("A child thread panicked"); // TODO: error logging dorkus!

    use std::time::{Instant, Duration};

    let start = Instant::now();
    let flat_buffer: Vec<u8> = img_buf.iter().flat_map(|array| array.iter()).cloned().collect();
    let duration = start.elapsed();

    println!("Time to collapse buffer: {:?}", duration);

    flat_buffer

}

pub fn renderloop(context: &RenderContext, world: &SphereWorld) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    
    let s_x = context.start_x as usize;
    let s_y = context.start_y as usize;
    let e_x = context.end_x as usize;
    let e_y = context.end_y as usize;

    let w = context.camera.film_width as usize;
    let h = context.camera.film_height as usize;
    let pixel_scale = 1.0 / (context.samples as f32);

    let mut img_buf = vec![0_u8; (w * h * 3) as usize];

    for y in s_y..e_y {
        for x in s_x..e_x {
            let pixel = render_pixel(context, world, x, y);

            // TODO: turn into macros
            img_buf[(3 * (y * w + x) + 0) as usize] = (clamp(f32::sqrt(pixel.x()), 0.0, 0.999) * 256.0) as u8;
            img_buf[(3 * (y * w + x) + 1) as usize] = (clamp(f32::sqrt(pixel.y()), 0.0, 0.999) * 256.0) as u8;
            img_buf[(3 * (y * w + x) + 2) as usize] = (clamp(f32::sqrt(pixel.z()), 0.0, 0.999) * 256.0) as u8;    
        }
    }

    img_buf
}


fn ray_color_it(r: &Ray, world: Arc<dyn Hittable>, depth: u32) -> Color {
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

fn ray_color_it_mt(r: &Ray, world: &SphereWorld, depth: u32) -> Color {
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
