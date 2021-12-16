use crate::renderables::world::HittableList;
use crate::core::camera::Camera;

pub struct RenderContext<'a> {
    pub camera: Camera,
    pub world: HittableList,
    pub max_depth: u32,
    pub buffer: &'a mut Vec<u8>,
    pub samples: u32,
    pub start_x: u32,
    pub start_y: u32,
    pub end_x: u32,
    pub end_y: u32,
}

impl<'a> RenderContext<'a> {

    pub fn new(buffer: &'a mut Vec<u8>, camera: Camera, world: HittableList, max_depth: u32, samples: u32, 
        start_x: u32, start_y: u32, end_x: u32, end_y: u32) -> RenderContext<'a> {
        RenderContext { camera, world, buffer, max_depth, samples, start_x, start_y, end_x, end_y }
    }
}