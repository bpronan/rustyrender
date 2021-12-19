//! This module contains the actual rendering implementation. 
//! As a library, it has a single entry point, 'render()'.

pub mod core;
pub mod scene;
mod execute;

use log::{info, error};
use std::time::{Instant};
use thiserror::Error;

use crate::renderer::execute::context::RenderContext;
use crate::renderer::execute::cpurender;
use crate::renderer::execute::gpurender;
use crate::renderer::scene::camera::Camera;

use image::RgbImage;
use crate::renderer::scene::world::Region;

/// The compute backend to use for the render. Naive
/// is a single threaded loop. Multicore will create a 
/// thread per machine logical CPU. Cuda and OpenCL use 
/// GPU or HW accelerated algorithms.
#[derive(Copy, Clone, Debug, serde::Deserialize)]
pub enum ComputeEnv {
    Naive,
    Multicore,
    Cuda,
    Opencl,
}

#[derive(Error, Debug)]
pub enum RendererError {

    #[error(transparent)]
    ComputeError(#[from] crate::renderer::execute::error::ComputeError ),

    #[error("You must provide a buffer of with dimensions greater than 0 or less than equal to 4096.")]
    BufferSizeError,

    #[error("Samples per pixel and max depth must be greater than 0.")]
    InvalidParameterError,

    #[error("Empty scene file.")]
    InvalidSceneError,


}


/// This macro takes an expression as an argument and will
/// log to error and panic on debug only. This is useful for 
/// precondition checks for external APIs.
macro_rules! condition_check {
    ($expression:expr, $error:expr) => {
        if($expression) {
            error!("API precondition check failed: {}", stringify!($expression));
            return Err($error);
        }
    };
}


/// Renders the objects in the world into the image buffer using a simple
/// diffuse raytracing algorithm. It will choose the most effective compute 
/// environment for the process based on the ComputeEnv suggestion.
/// 
/// # Arguments
/// 
/// * `env` - The suggested backend compute environment. It will always choose
/// a environment that will succeed.
/// * `samples_per_pixel` - The number of samples to use for antialiasing. Must be > 0.
/// * `max_depth` - The maximum number of ray reflections to use per pixel. Must be > 0.
/// * `aspect_ratio` - The output aspect ratio.
/// * `world` - The scene as a HittableList object.
/// * `img_bug` - The output image buffer. Must be of type rgb and have
/// a width and height of the intended output image. Width and height must be
/// in (0, 4096].
/// 
/// # Examples
/// 
/// ```
/// use parser::FileReaderFactory;
/// use renderer::ComputeEnv;
/// 
/// let world = file_parser.process_file()?;
/// let mut img: RgbImage = ImageBuffer::new(imgx, imgy);
/// renderer::render(compute_env,
///    imgx, imgy, samples_per_pixel, max_depth, aspect_ratio,
///    &world, &mut img)?;
/// img.save(&args.arg_dest)?;
/// ```
/// 
/// REVIEW: The above example will cause 'cargo test' to fail. I'm struggling 
/// getting the module wrangling working within the example code, so in the 
/// meantime, run 'cargo test --lib'.
/// 
/// # Errors
/// 
/// * `InvalideParameterError` - A parameter is within an invalid range.
/// * `BufferSizeError` - The image buffer supplied is of an unsupported size.
/// * `ComputeError` - There was an error or panic while executing the render.
/// This is likely due to a system level failure or defect.
pub fn render(env: ComputeEnv, samples_per_pixel: u32, max_depth: u32, 
    world: &Region, img_buf: &mut RgbImage) -> Result<(), RendererError> {

    // precondition checks
    condition_check!(samples_per_pixel == 0, 
        RendererError::InvalidParameterError);
    condition_check!(max_depth == 0, 
        RendererError::InvalidParameterError);
    condition_check!(img_buf.width() == 0 || img_buf.height() == 0, 
        RendererError::BufferSizeError);
    condition_check!(img_buf.height() > 4096 || img_buf.height() > 4096, 
        RendererError::BufferSizeError);
    condition_check!(world.objects.len() == 0, 
        RendererError::InvalidSceneError);

    let width = img_buf.width();
    let height = img_buf.height();
    let aspect_ratio = width as f32 / height as f32;

    let context = RenderContext::new( 
            Camera::new(2.0, 2.0 * aspect_ratio, 1.0, width, height), 
            max_depth, samples_per_pixel, 0, 0, width, height
    );
    
    let start = Instant::now();

    match env {
        ComputeEnv::Naive => {
            info!("Executing naive implementation.");
            cpurender::render_naive(&context, &world, img_buf);
        },
        ComputeEnv::Cuda => {
            info!("Executing CUDA implementation.");
            gpurender::render_cuda(&context, &world, img_buf)?;
        },
        ComputeEnv::Opencl => {
            info!("Executing OpenCL implementation.");
            gpurender::render_opencl(&context, &world, img_buf)?
        },
        _ => {
            info!("Executing Mulithreading implementation.");
            cpurender::render_threaded(&context, &world, img_buf)?
        },
    };
    info!("Rendering execution time: {:?}", start.elapsed());

    // post condition checks go here ;-)

    Ok(())
}
