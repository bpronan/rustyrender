use log::error;
use crate::renderer::core::debug_check;
use crate::renderer::scene::camera::Camera;

/// A simple container struct for the information needed to 
/// render the image. The lifetime of this will exist for the entire
/// image process. This will probably have to be extended for GPU
/// to contain the context, framebuffers, and etc.
pub struct RenderContext {
    pub camera: Camera,
    pub max_depth: u32,
    pub samples: u32,
    pub start_x: u32,
    pub start_y: u32,
    pub end_x: u32,
    pub end_y: u32,
}

impl RenderContext {

    /// Create a new RenderContext.
    /// 
    /// Parameters:
    /// * `camera` - The camera object to render.
    /// * `max_depth` - The maximum number of ray reflections to use per pixel. Must be > 0.
    /// * `samples_per_pixel` - The number of samples to use for antialiasing. Must be > 0.
    /// * `start_x` - The starting pixel x coordinate for this render
    /// * `start_y` - The starting pixel y coordinate for this render
    /// * `end_x` - The ending pixel x coordinate for this render
    /// * `end_y` - The ending pixel y coordinate for this render
    pub fn new(camera: Camera, max_depth: u32, samples: u32, 
        start_x: u32, start_y: u32, end_x: u32, end_y: u32) -> RenderContext {

        // these values should be verified on the API interface, adding asserts
        // here to prevent anyone from unintentionally removing those checks.
        debug_check!(max_depth != 0);
        debug_check!(samples != 0);
        debug_check!(start_x != end_x);
        debug_check!(start_y != end_y);

        RenderContext { camera, max_depth, samples, start_x, start_y, end_x, end_y }
    }
}
