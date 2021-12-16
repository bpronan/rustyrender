use crate::core::vector::Color;

use crate::renderables::world;
use super::context::RenderContext;

use rand::Rng;


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

pub fn renderloop(context: RenderContext) {
    let mut rng = rand::thread_rng();
    
    let s_x = context.start_x;
    let s_y = context.start_y;
    let e_x = context.end_x;
    let e_y = context.end_y;

    let w = context.camera.film_width;
    let h = context.camera.film_height;
    let pixel_scale = 1.0 / (context.samples as f32);

    for y in s_y..e_y {
        for x in s_x..e_x {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
    
            for _n in 0..context.samples {
                let randx = rng.gen_range(0.0..1.0);
                let randy = rng.gen_range(0.0..1.0);
                let u = ((x as f32) + randx) / ((w - 1) as f32);
                let v = (((h - y) as f32) + randy) / ((h - 1) as f32);
    
                let r = context.camera.get_ray(u, v);
    
                pixel_color += world::ray_color(&r, &context.world, context.max_depth);
            }
    
            pixel_color *= pixel_scale;

            // TODO: turn into macros
            context.buffer[(3 * (y * w + x) + 0) as usize] = (clamp(f32::sqrt(pixel_color.x()), 0.0, 0.999) * 256.0) as u8;
            context.buffer[(3 * (y * w + x) + 1) as usize] = (clamp(f32::sqrt(pixel_color.y()), 0.0, 0.999) * 256.0) as u8;
            context.buffer[(3 * (y * w + x) + 2) as usize] = (clamp(f32::sqrt(pixel_color.z()), 0.0, 0.999) * 256.0) as u8;    
        }
    }
}

