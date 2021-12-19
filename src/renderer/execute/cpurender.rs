use crate::renderer::core::debug_check;
use crate::renderer::scene::world::Region;

use log::error;
use image::{Rgb, RgbImage};

use std::sync::Arc;
use std::sync::mpsc;

use super::context::RenderContext;
use super::error::ComputeError;
use super::util;

/// Renders the multicore version of the algorithm using a set of n threads
/// and a mpsc channel to collect the pixels into the output.
/// 
/// Parameters:
/// * `context` - The render context that contains the information necessary to render the image.
/// * `world` - The scene to render.
/// * `img` - The image buffer to write to.
pub fn render_threaded(context: &RenderContext, 
    world: &Region, 
    img: &mut RgbImage) -> Result<(), ComputeError> {

    // already covered by checks on the public api, but here to keep the internal behavior 
    // consistency
    debug_check!(world.objects.len() > 0);
    debug_check!(img.width() > 0);
    debug_check!(img.height() > 0);

    let ctx_arc = Arc::new(context);
    let wrld_arc = Arc::new(world);

    let w = ctx_arc.camera.film_width;
    let h = ctx_arc.camera.film_height;

    let cpus = num_cpus::get() as u32;

    let (tx, rx) = mpsc::channel();

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

                        let pixel = util::render_pixel(&ctx, &world, x as usize, y as usize);

                        // REVIEW: would love to turn this into a macro, if only there were time.
                        let r = (util::clamp(f32::sqrt(pixel.x), 0.0, 0.999) * 256.0) as u8;
                        let g = (util::clamp(f32::sqrt(pixel.y), 0.0, 0.999) * 256.0) as u8;
                        let b = (util::clamp(f32::sqrt(pixel.z), 0.0, 0.999) * 256.0) as u8;
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
pub fn render_naive(context: &RenderContext, world: &Region, img: &mut RgbImage) {

    // already covered by checks on the public api, but here to keep the internal 
    // consistency
    debug_check!(world.objects.len() > 0);
    debug_check!(img.width() > 0);
    debug_check!(img.height() > 0);

    let s_x = context.start_x;
    let s_y = context.start_y;
    let e_x = context.end_x;
    let e_y = context.end_y;

    for y in s_y..e_y {
        for x in s_x..e_x {
            let pixel = util::render_pixel(context, world, x as usize, y as usize);

            let r = (util::clamp(f32::sqrt(pixel.x), 0.0, 0.999) * 256.0) as u8;
            let g = (util::clamp(f32::sqrt(pixel.y), 0.0, 0.999) * 256.0) as u8;
            let b = (util::clamp(f32::sqrt(pixel.z), 0.0, 0.999) * 256.0) as u8;    
            img.put_pixel(x, y, Rgb([r, g, b]));

        }
    }
}



