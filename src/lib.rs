mod core;
mod execute;
mod io;
mod renderables;

use log::info;

use crate::execute::context::RenderContext;
use crate::execute::renderloop;
use crate::io::FileReaderFactory;
use crate::io::image;
use crate::core::camera::Camera;

use std::env;
use std::error::Error;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // TODO magic numbers
    let aspect_ratio = 16.0 / 9.0;
    let imgx: u32 = 400;
    let imgy: u32 = ((imgx as f32) / aspect_ratio) as u32;
    let samples_per_pixel = 10;
    let max_depth = 50;

    let file_parser = FileReaderFactory::get_file_processor(&"world.json".to_string())
        .expect("Couldn't retrieve file parser");
    let world = file_parser.process_file();

    info!("World successfully built!");

    // camera

    use std::time::{Duration, Instant};

    // let start = Instant::now();
    // let flat_buffer = renderloop::renderloop(
    //     &RenderContext::new( 
    //         Camera::new(2.0, 2.0 * aspect_ratio, 1.0, imgx, imgy), 
    //         max_depth, samples_per_pixel, 0, 0, imgx, imgy
    //     ), &world
    // );
    // let duration = start.elapsed();
    // info!("Flat execution time: {:?}", duration);


    let start = Instant::now();
    let flat_buffer = renderloop::renderloop_concurrent(
        &RenderContext::new( 
            Camera::new(2.0, 2.0 * aspect_ratio, 1.0, imgx, imgy), 
            max_depth, samples_per_pixel, 0, 0, imgx, imgy
        ), &world
    );
    let duration = start.elapsed();
    info!("Concurrent execution time: {:?}", duration);


    image::write_image_to_file("output/render0.png".to_string(), &flat_buffer, imgx as usize, imgy as usize);

    Ok(())
}

pub struct Config {
    input_file: String,
    output_file: String,
}

pub fn usage() {
    println!("Usage: {} input_file output_file", 
        env!("CARGO_PKG_NAME"));

}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let input_file = args[1].clone();
        let output_file = args[2].clone();

        Ok(Config { input_file, output_file })
    }
}