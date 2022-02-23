//! This module contains the actual rendering implementation.
//! As a library, it has a single entry point, 'render()'.

pub mod core;
mod execute;
pub mod scene;

use log::{error, info};
use std::time::Instant;
use thiserror::Error;

use crate::renderer::execute::context::RenderContext;
use crate::renderer::execute::cpurender;
use crate::renderer::execute::gpurender;
use crate::renderer::execute::render_op;
use crate::renderer::scene::camera::Camera;
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
    SimpleThreaded,
}

#[derive(Error, Debug)]
pub enum RendererError {
    #[error(transparent)]
    ComputeError(#[from] crate::renderer::execute::error::ComputeError),

    #[error(
        "You must provide a buffer of with dimensions greater than 0 or less than equal to 4096."
    )]
    BufferSize,

    #[error("Samples per pixel and max depth must be greater than 0.")]
    InvalidParameter,

    #[error("Empty scene file.")]
    InvalidScene,
}

/// This macro takes an expression as an argument and will
/// log to error and panic on debug only. This is useful for
/// precondition checks for external APIs.
#[cfg(not(debug_assertions))]
macro_rules! condition_check {
    ($expression:expr, $error:expr) => {
        if ($expression) {
            error!("API precondition check failed: {}", stringify!($expression));
            return Err($error);
        }
    };
}

#[cfg(debug_assertions)]
macro_rules! condition_check {
    ($expression:expr, $error:expr) => {
        if ($expression) {
            panic!(
                "API precondition check failed: {}, error code: {}",
                stringify!($expression),
                $error
            );
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
/// # use rustyrender::parser::FileReaderFactory;
/// # use rustyrender::renderer::{ComputeEnv, render};
/// # let input_file_path = "test_input/one_sphere.json";
/// # let file_parser = FileReaderFactory::get_file_processor(input_file_path).unwrap();
///
/// let world = file_parser.process_file().unwrap();
/// let imgx = 200;
/// let imgy = 200;
/// let mut pixels = vec![0; (imgx as usize) * (imgy as usize) * 3];
///
/// render(ComputeEnv::Multicore,
///    10, 50,
///    &world, &mut pixels, (imgx, imgy));
/// ```
///
/// # Errors
///
/// * `InvalideParameterError` - A parameter is within an invalid range.
/// * `BufferSizeError` - The image buffer supplied is of an unsupported size.
/// * `ComputeError` - There was an error or panic while executing the render.
/// This is likely due to a system level failure or defect.
pub fn render(
    env: ComputeEnv,
    samples_per_pixel: u32,
    max_depth: u32,
    world: &Region,
    pixels: &mut [u8],
    bounds: (u32, u32),
) -> Result<(), RendererError> {
    // precondition checks
    condition_check!(samples_per_pixel == 0, RendererError::InvalidParameter);
    condition_check!(max_depth == 0, RendererError::InvalidParameter);
    condition_check!(bounds.0 == 0 || bounds.1 == 0, RendererError::BufferSize);
    condition_check!(
        bounds.0 > 4096 || bounds.1 > 4096,
        RendererError::BufferSize
    );
    condition_check!(world.objects.is_empty(), RendererError::InvalidScene);

    let camera_config = world.camera_config;

    let context = RenderContext::new(
        Camera::new(camera_config, bounds),
        max_depth,
        samples_per_pixel,
        0,
        0,
        bounds.0,
        bounds.1,
    );

    let start = Instant::now();

    match env {
        ComputeEnv::Naive => {
            info!("Executing naive implementation.");
            cpurender::render_naive(&context, world, pixels, bounds, render_op::render_pixel);
        }
        ComputeEnv::Cuda => {
            info!("Executing CUDA implementation.");
            gpurender::render_cuda(&context, world, pixels, bounds)?;
        }
        ComputeEnv::Opencl => {
            info!("Executing OpenCL implementation.");
            gpurender::render_opencl(&context, world, pixels, bounds)?
        }
        ComputeEnv::SimpleThreaded => {
            info!("Executing naive multithreaded implementation.");
            cpurender::render_threaded_naive(
                &context,
                world,
                pixels,
                bounds,
                render_op::render_pixel,
            )?
        }
        _ => {
            info!("Executing Mulithreading implementation.");
            cpurender::render_threaded(&context, world, pixels, bounds, render_op::render_pixel)?
        }
    };
    info!("Rendering execution time: {:?}", start.elapsed());

    // post condition checks go here ;-)

    Ok(())
}

impl std::fmt::Display for ComputeEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ComputeEnv::Cuda => write!(f, "cuda"),
            ComputeEnv::Naive => write!(f, "naive"),
            ComputeEnv::Multicore => write!(f, "multicore"),
            ComputeEnv::Opencl => write!(f, "opencl"),
            ComputeEnv::SimpleThreaded => write!(f, "simple_threaded"),
        }
    }
}
