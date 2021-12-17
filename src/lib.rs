mod core;
mod execute;
mod io;
mod renderables;


use crate::execute::context::RenderContext;
use crate::execute::renderloop;
use crate::io::FileReaderFactory;
use crate::core::camera::Camera;

use image::{ImageBuffer, RgbImage};
use log::{info, error};
use std::env;
use std::error::Error;
use std::time::{Instant};


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // TODO magic numbers
    let aspect_ratio = 16.0 / 9.0;
    let imgx: u32 = 1920;
    let imgy: u32 = ((imgx as f32) / aspect_ratio) as u32;
    let samples_per_pixel = 10;
    let max_depth = 50;

    let file_parser = FileReaderFactory::get_file_processor(&config.input_file)
        .expect("Couldn't retrieve file parser");
    let world = file_parser.process_file();

    info!("World successfully built!");

    let mut img: RgbImage = ImageBuffer::new(imgx, imgy);

    let start = Instant::now();
    renderloop::renderloop(
        &RenderContext::new( 
            Camera::new(2.0, 2.0 * aspect_ratio, 1.0, imgx, imgy), 
            max_depth, samples_per_pixel, 0, 0, imgx, imgy
        ), &world, &mut img
    );
    let duration = start.elapsed();
    info!("Flat execution time: {:?}", duration);


    let start = Instant::now();
    match renderloop::renderloop_concurrent(
        &RenderContext::new( 
            Camera::new(2.0, 2.0 * aspect_ratio, 1.0, imgx, imgy), 
            max_depth, samples_per_pixel, 0, 0, imgx, imgy
        ), &world, &mut img
    ) {
        Err(e) => error!("{:?}", e),
        _ => ()
    }
    let duration = start.elapsed();
    info!("Concurrent execution time: {:?}", duration);

    img.save(&config.output_file).expect("Could not write output file");

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