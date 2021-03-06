use crate::renderer::core::{color::Color, convert_pixel, debug_check, write_pixel};
use crate::renderer::scene::world::Region;

use log::error;

use super::context::RenderContext;
use super::error::ComputeError;

use rayon::prelude::*;

type RenderPixelOp = fn(&RenderContext, &Region, usize, usize) -> Color;

/// Renders the multicore version of the algorithm using a set of n threads
/// and a mpsc channel to collect the pixels into the output.
///
/// Parameters:
/// * `context` - The render context that contains the information necessary to render the image.
/// * `world` - The scene to render.
/// * `img` - The image buffer to write to.
pub fn render_threaded(
    context: &RenderContext,
    world: &Region,
    pixels: &mut [u8],
    bounds: (u32, u32),
    render_op: RenderPixelOp,
) -> Result<(), ComputeError> {
    // already covered by checks on the public api, but here to keep the internal behavior
    // consistency
    debug_check!(bounds.0 > 0);
    debug_check!(bounds.1 > 0);
    debug_check!(pixels.len() == (bounds.0 as usize * bounds.1 as usize * 3));

    let w = bounds.0 as usize;

    //let mut pixels = img.into_raw();
    let bands: Vec<(usize, &mut [u8])> = pixels.chunks_mut(w * 3).enumerate().collect();

    bands.into_par_iter().for_each(|(i, band)| {
        for x in 0..w {
            let pixel = render_op(context, world, x as usize, i as usize);

            write_pixel!(pixel, band, x);
        }
    });

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
pub fn render_naive(
    context: &RenderContext,
    world: &Region,
    pixels: &mut [u8],
    bounds: (u32, u32),
    render_op: RenderPixelOp,
) {
    // already covered by checks on the public api, but here to keep the internal
    // consistency
    debug_check!(bounds.0 > 0);
    debug_check!(bounds.1 > 0);

    let s_x = context.start_x;
    let s_y = context.start_y;
    let e_x = context.end_x;
    let e_y = context.end_y;

    let w = (e_x - s_x) as usize;

    for y in s_y..e_y {
        for x in s_x..e_x {
            let pixel = render_op(context, world, x as usize, y as usize);

            write_pixel!(pixel, pixels, (y as usize * w + x as usize));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::core::{color::Color, vector::Vec3};
    use crate::renderer::scene::camera::{Camera, CameraConfig};
    use crate::renderer::scene::world::Region;
    use image::{ImageBuffer, RgbImage};

    // Render a bizarre value out for testing purposes.
    pub fn render_test_pixel(
        _ctx_arc: &RenderContext,
        _world: &Region,
        x: usize,
        y: usize,
    ) -> Color {
        println!("{} {}", x, y);
        Color::new(0.013 * (x as f32), 0.017 * (y as f32), 0.21)
    }

    #[test]
    #[should_panic]
    fn test_naive_empty_image() {
        let mut img = vec![0; 0];
        let ctx = RenderContext::new(
            Camera::new(
                CameraConfig {
                    look_from: Vec3::new(0.0, 0.0, 0.0),
                    look_at: Vec3::new(0.0, 0.0, -1.0),
                    up: Vec3::new(0.0, 1.0, 0.0),
                    aperture: 2.0,
                    focal_distance: 1.0,
                    vertical_fov: 90.0,
                },
                (2, 2),
            ),
            1,
            1,
            0,
            0,
            2,
            2,
        );

        let r = Region::new(Color::new(0.13, 0.17, 0.23));

        render_naive(&ctx, &r, &mut img, (2, 2), render_test_pixel);
    }

    #[test]
    #[should_panic]
    fn test_empty_image() {
        let mut img = vec![0; 0];
        let ctx = RenderContext::new(
            Camera::new(
                CameraConfig {
                    look_from: Vec3::new(0.0, 0.0, 0.0),
                    look_at: Vec3::new(0.0, 0.0, -1.0),
                    up: Vec3::new(0.0, 1.0, 0.0),
                    aperture: 2.0,
                    focal_distance: 1.0,
                    vertical_fov: 90.0,
                },
                (2, 2),
            ),
            1,
            1,
            0,
            0,
            2,
            2,
        );

        let r = Region::new(Color::new(0.13, 0.17, 0.23));

        match render_threaded(&ctx, &r, &mut img, (2, 2), render_test_pixel) {
            Err(_) => panic!("we expect this"),
            _ => {}
        }
    }

    #[test]
    fn test_render_full_naive() {
        let mut img = vec![0; 2 * 2 * 3];
        let ctx = RenderContext::new(
            Camera::new(
                CameraConfig {
                    look_from: Vec3::new(0.0, 0.0, 0.0),
                    look_at: Vec3::new(0.0, 0.0, -1.0),
                    up: Vec3::new(0.0, 1.0, 0.0),
                    aperture: 2.0,
                    focal_distance: 1.0,
                    vertical_fov: 90.0,
                },
                (2, 2),
            ),
            1,
            1,
            0,
            0,
            2,
            2,
        );
        let r = Region::new(Color::new(1.0, 1.0, 1.0));

        render_naive(&ctx, &r, &mut img, (2, 2), render_test_pixel);

        for y in 0..2 {
            for x in 0..2 {
                let (r, g, b) = convert_pixel(Vec3 {
                    x: 0.013 * (x as f32),
                    y: 0.017 * (y as f32),
                    z: 0.21,
                });
                assert_eq!(img[3 * (2 * y + x) + 0], r);
                assert_eq!(img[3 * (2 * y + x) + 1], g);
                assert_eq!(img[3 * (2 * y + x) + 2], b);
            }
        }
    }

    #[test]
    fn test_render_full_threaded() {
        let mut img: RgbImage = ImageBuffer::new(2, 2);
        let ctx = RenderContext::new(
            Camera::new(
                CameraConfig {
                    look_from: Vec3::new(0.0, 0.0, 0.0),
                    look_at: Vec3::new(0.0, 0.0, -1.0),
                    up: Vec3::new(0.0, 1.0, 0.0),
                    aperture: 2.0,
                    focal_distance: 1.0,
                    vertical_fov: 90.0,
                },
                (2, 2),
            ),
            1,
            1,
            0,
            0,
            2,
            2,
        );
        let r = Region::new(Color::new(1.0, 1.0, 1.0));

        match render_threaded(&ctx, &r, &mut img, (2, 2), render_test_pixel) {
            Err(_) => panic!("we don't expect this"),
            _ => {}
        }

        for (x, y, pixel) in img.enumerate_pixels() {
            let (r, g, b) = convert_pixel(Vec3 {
                x: 0.013 * (x as f32),
                y: 0.017 * (y as f32),
                z: 0.21,
            });
            assert_eq!(pixel[0], r);
            assert_eq!(pixel[1], g);
            assert_eq!(pixel[2], b);
        }
    }
}
