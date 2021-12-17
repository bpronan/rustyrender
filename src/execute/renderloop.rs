use crate::core::ray::Ray;
use crate::core::vector;
use crate::core::vector::{ Color, Point3, Vec3 };
use crate::renderables::world::{ Hittable, HitRecord };
use crate::renderables::world::HittableList;

use image::{Rgb, RgbImage};
use rand::Rng;

use std::sync::Arc;
use std::sync::mpsc::{channel, RecvError};

use super::context::RenderContext;



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



fn render_pixel(ctx_arc: &RenderContext, world: &HittableList, x: usize, y: usize) -> Color {
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


pub fn renderloop_concurrent(context: &RenderContext, 
    world: &HittableList, 
    img: &mut RgbImage) -> Result<(), RecvError> {

    let ctx_arc = Arc::new(context);
    let wrld_arc = Arc::new(world);

    let w = ctx_arc.camera.film_width;
    let h = ctx_arc.camera.film_height;

    let cpus = num_cpus::get() as u32;


    let (tx, rx) = channel();

    crossbeam::scope(|scope| {
        for n in 0..cpus {
            let lenx = w / cpus;
            let leny = h;
            let world = wrld_arc.clone();
            let ctx = ctx_arc.clone();
            let tx = tx.clone();

            scope.spawn(move |_| {
                for x in n*lenx..(n+1)*lenx {
                    for y in 0..leny {

                        let pixel = render_pixel(&ctx, &world, x as usize, y as usize);
                        let r = (clamp(f32::sqrt(pixel.x()), 0.0, 0.999) * 256.0) as u8;
                        let g = (clamp(f32::sqrt(pixel.y()), 0.0, 0.999) * 256.0) as u8;
                        let b = (clamp(f32::sqrt(pixel.z()), 0.0, 0.999) * 256.0) as u8;
                        tx.send((x, y, Rgb([r, g, b]))).unwrap();

                    }
                }
            });

        }
    }).expect("A child thread panicked");

    for _ in 0..(w * h) {
        let (x, y, pixel) = rx.recv()?;
        img.put_pixel(x as u32, y as u32, pixel);
    }

    Ok(())
}

pub fn renderloop(context: &RenderContext, world: &HittableList, img: &mut RgbImage) {    
    let s_x = context.start_x;
    let s_y = context.start_y;
    let e_x = context.end_x;
    let e_y = context.end_y;

    for y in s_y..e_y {
        for x in s_x..e_x {
            let pixel = render_pixel(context, world, x as usize, y as usize);

            let r = (clamp(f32::sqrt(pixel.x()), 0.0, 0.999) * 256.0) as u8;
            let g = (clamp(f32::sqrt(pixel.y()), 0.0, 0.999) * 256.0) as u8;
            let b = (clamp(f32::sqrt(pixel.z()), 0.0, 0.999) * 256.0) as u8;    
            img.put_pixel(x, y, Rgb([r, g, b]));

        }
    }
}


fn ray_color_it(r: &Ray, world: &HittableList, depth: u32) -> Color {
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
