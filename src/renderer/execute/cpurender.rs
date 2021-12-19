use crate::renderer::core::debug_check;
use crate::renderer::core::color::Color;
use crate::renderer::scene::world::Region;

use log::error;
use image::{Rgb, RgbImage};

use std::sync::Arc;
use std::sync::mpsc;

use super::context::RenderContext;
use super::error::ComputeError;
use super::render_op;

type RenderPixelOp = fn(&RenderContext, &Region, usize, usize) -> Color;

/// Renders the multicore version of the algorithm using a set of n threads
/// and a mpsc channel to collect the pixels into the output.
/// 
/// Parameters:
/// * `context` - The render context that contains the information necessary to render the image.
/// * `world` - The scene to render.
/// * `img` - The image buffer to write to.
pub fn render_threaded(context: &RenderContext, 
    world: &Region, 
    img: &mut RgbImage, render_op: RenderPixelOp) -> Result<(), ComputeError> {

    // already covered by checks on the public api, but here to keep the internal behavior 
    // consistency
    debug_check!(img.width() > 0);
    debug_check!(img.height() > 0);

    let ctx_arc = Arc::new(context);
    let wrld_arc = Arc::new(world);

    let w = ctx_arc.camera.film_width;
    let h = ctx_arc.camera.film_height;

    // we don't want to create more threads than slices
    let threads = std::cmp::min(num_cpus::get() as u32, w);

    let (tx, rx) = mpsc::channel();

    crossbeam::scope(|scope| {
        for n in 0..threads {
            let lenx = w / threads;
            let leny = h;
            let start_x = n * lenx;
            // leave the last thread to handle the left over if the math doesn't work out
            let end_x = if n == threads- 1 { w } else { (n+1) * lenx };

            let world = wrld_arc.clone();
            let ctx = ctx_arc.clone();
            let tx = tx.clone();

            scope.spawn(move |_| {
                for x in start_x..end_x {
                    for y in 0..leny {

                        let pixel = render_op(&ctx, &world, x as usize, y as usize);

                        // REVIEW: would love to turn this into a macro, if only there were time.
                        let r = (render_op::clamp(f32::sqrt(pixel.x), 0.0, 0.999) * 256.0) as u8;
                        let g = (render_op::clamp(f32::sqrt(pixel.y), 0.0, 0.999) * 256.0) as u8;
                        let b = (render_op::clamp(f32::sqrt(pixel.z), 0.0, 0.999) * 256.0) as u8;
                        tx.send((x, y, Rgb([r, g, b]))).unwrap();

                    }
                }
            });
        }

    }).map_err(|source| {
        error!("Thread error: {:?}", source);
        ComputeError::ThreadPanickedError 
    })?;

    for _ in 0..(w * h) {
        let (x, y, pixel) = rx.recv()?;
        img.put_pixel(x as u32, y as u32, pixel);
    }

    Ok(())
}


/// Renders the naive algorithm. It's a simple single threaded loop over all the 
/// pixels in the image. This is not likely to be used within the scope of this project,
/// however, if it were to be extended to where the tiling and threading were handled
/// on a per process basis, this could be useful. Additionally, it's useful as a 
/// testing and performance baseline.
/// 
/// Parameters:
/// * `context` - The render context that contains the information necessary to render the image.
/// * `world` - The scene to render.
/// * `img` - The image buffer to write to.
pub fn render_naive(context: &RenderContext, world: &Region, img: &mut RgbImage, render_op: RenderPixelOp) {

    // already covered by checks on the public api, but here to keep the internal 
    // consistency
    debug_check!(img.width() > 0);
    debug_check!(img.height() > 0);

    let s_x = context.start_x;
    let s_y = context.start_y;
    let e_x = context.end_x;
    let e_y = context.end_y;

    for y in s_y..e_y {
        for x in s_x..e_x {
            let pixel = render_op(context, world, x as usize, y as usize);

            let r = (render_op::clamp(f32::sqrt(pixel.x), 0.0, 0.999) * 256.0) as u8;
            let g = (render_op::clamp(f32::sqrt(pixel.y), 0.0, 0.999) * 256.0) as u8;
            let b = (render_op::clamp(f32::sqrt(pixel.z), 0.0, 0.999) * 256.0) as u8;    
            img.put_pixel(x, y, Rgb([r, g, b]));

        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::scene::camera::Camera;
    use crate::renderer::scene::world::Region;
    use crate::renderer::core::color::Color;
    use image::{RgbImage, ImageBuffer};

    // Render a bizarre value out for testing purposes.
    pub fn render_test_pixel(
        _ctx_arc: &RenderContext, 
        _world: &Region, 
        x: usize, y: usize) -> Color {
        Color::new(0.013 * (x as f32), 0.017 * (y as f32), 0.21)
    }

    #[test]
    #[should_panic]
    fn test_naive_empty_image() {
        let mut img: RgbImage = ImageBuffer::new(0, 0);
        let ctx = RenderContext::new(Camera::new(1.0, 1.0, 1.0, 2, 2), 1, 1, 0, 0, 2, 2);

        let r = Region::new(Color::new(0.13, 0.17, 0.23));

        render_naive(&ctx, &r, &mut img, render_test_pixel);
    }

    #[test]
    #[should_panic]
    fn test_empty_image() {
        let mut img: RgbImage = ImageBuffer::new(0, 0);
        let ctx = RenderContext::new(Camera::new(1.0, 1.0, 1.0, 2, 2), 1, 1, 0, 0, 2, 2);

        let r = Region::new(Color::new(0.13, 0.17, 0.23));

        match render_threaded(&ctx, &r, &mut img, render_test_pixel) {
            Err(_) => panic!("we expect this"),
            _ => {}
        }
    }

    #[test]
    fn test_render_full_naive() {
        let mut img: RgbImage = ImageBuffer::new(2, 2);
        let ctx = RenderContext::new(Camera::new(1.0, 1.0, 1.0, 2, 2), 1, 1, 0, 0, 2, 2);

        let r = Region::new(Color::new(1.0, 1.0, 1.0));

        render_naive(&ctx, &r, &mut img, render_test_pixel);

        for (x, y, pixel) in img.enumerate_pixels() {
            assert_eq!(pixel[0], (render_op::clamp(f32::sqrt(0.013 * (x as f32)), 0.0, 0.999) * 256.0) as u8);
            assert_eq!(pixel[1], (render_op::clamp(f32::sqrt(0.017 * (y as f32)), 0.0, 0.999) * 256.0) as u8);
            assert_eq!(pixel[2], (render_op::clamp(f32::sqrt(0.21), 0.0, 0.999) * 256.0) as u8);
        }
    }


    #[test]
    fn test_render_full_threaded() {
        let mut img: RgbImage = ImageBuffer::new(2, 2);
        let ctx = RenderContext::new(Camera::new(1.0, 1.0, 1.0, 2, 2), 1, 1, 0, 0, 2, 2);

        let r = Region::new(Color::new(1.0, 1.0, 1.0));

        match render_threaded(&ctx, &r, &mut img, render_test_pixel) {
            Err(_) => panic!("we don't expect this"),
            _ => {}
        }

        for (x, y, pixel) in img.enumerate_pixels() {
            assert_eq!(pixel[0], (render_op::clamp(f32::sqrt(0.013 * (x as f32)), 0.0, 0.999) * 256.0) as u8);
            assert_eq!(pixel[1], (render_op::clamp(f32::sqrt(0.017 * (y as f32)), 0.0, 0.999) * 256.0) as u8);
            assert_eq!(pixel[2], (render_op::clamp(f32::sqrt(0.21), 0.0, 0.999) * 256.0) as u8);
        }
    }

    // Bugfix: <bug tracking number>: this test will hang if the bug is reintroduced
    // Review: This shows my basic maintenance workflow, 
    // this test was added first to reproduce the problem, then the fix was added.
    #[test]
    fn test_render_width_not_multiple_of_threads() {

        let cpus = num_cpus::get() as u32;
        let w = cpus * 10 + (cpus - 1);
        let h = w;
        let mut img: RgbImage = ImageBuffer::new(w, h);
        let ctx = RenderContext::new(Camera::new(1.0, 1.0, 1.0, w, h), 1, 1, 0, 0, w, h);

        let r = Region::new(Color::new(1.0, 1.0, 1.0));

        match render_threaded(&ctx, &r, &mut img, render_test_pixel) {
            Err(_) => panic!("we don't expect this"),
            _ => {}
        }

        for (x, y, pixel) in img.enumerate_pixels() {
            assert_eq!(pixel[0], (render_op::clamp(f32::sqrt(0.013 * (x as f32)), 0.0, 0.999) * 256.0) as u8);
            assert_eq!(pixel[1], (render_op::clamp(f32::sqrt(0.017 * (y as f32)), 0.0, 0.999) * 256.0) as u8);
            assert_eq!(pixel[2], (render_op::clamp(f32::sqrt(0.21), 0.0, 0.999) * 256.0) as u8);
        }
    }

}