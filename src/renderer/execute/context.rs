use crate::renderer::scene::camera::Camera;

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

    pub fn new(camera: Camera, max_depth: u32, samples: u32, 
        start_x: u32, start_y: u32, end_x: u32, end_y: u32) -> RenderContext {
        RenderContext { camera, max_depth, samples, start_x, start_y, end_x, end_y }
    }
}
