use crate::renderer::scene::world::Region;

use super::context::RenderContext;
use super::error::ComputeError;
use image::RgbImage;
use log::warn;

/// A paper thin facade for a CUDA based render. This would be the
/// entry point for adding that feature set.
pub fn render_cuda(
    context: &RenderContext,
    world: &Region,
    img: &mut RgbImage,
) -> Result<(), ComputeError> {
    // NOTE: when adding CUDA support, make sure to use this code
    // to fall back in case the user isn't on a nVidia card.
    warn!("CUDA not supported yet, reverting to multithreaded CPU.");

    super::cpurender::render_threaded(context, world, img, super::render_op::render_pixel)?;

    Ok(())
}

/// A paper thin facade for a CUDA based render. This would be the
/// entry point for adding that feature set.
pub fn render_opencl(
    context: &RenderContext,
    world: &Region,
    img: &mut RgbImage,
) -> Result<(), ComputeError> {
    // NOTE: when adding CUDA support, make sure to use this code
    // to fall back in case the user isn't on an OpenCL environment.
    warn!("OpenCL not supported yet, reverting to multithreaded CPU.");

    super::cpurender::render_threaded(context, world, img, super::render_op::render_pixel)?;

    Ok(())
}
