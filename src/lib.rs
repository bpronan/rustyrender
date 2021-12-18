

use image::{ImageBuffer, RgbImage};
use log::info;

use anyhow::Context;

mod renderer;
mod parser;

use crate::parser::FileReaderFactory;
use renderer::ComputeEnv;

/// The usage string on which docopt will base the argument
/// parsing. 
/// 
/// Note: This should really be in main.rs instead of here.
/// It is in this file and not in main.rs for no other reason 
/// than that it wouldn't work with my weak Rust programming skills.
pub const USAGE: &str = "
Usage: rustyrender [options] <source> <dest>
       rustyrender --help

A simple renderer written in rust. 

Supported compute environments are:
    naive       A naive compute implementation.
    multicore   Parallelizes across cores on the CPU
    cuda        GP GPU based renderer using CUDA. Only supported on nVidia.
    opencl      OpenGL based renderer.

Options:
    -h, --help          Show this message.
    --compute <arg>     The environment to use on this machine.
    --width <arg>       The width of the output image. [default: 1920]
    --height <arg>      The height of the output image. [default: 1080]
    --samples <arg>     The number of antialiasing samples per pixel. [default: 10]
    --depth <arg>       The maximum depth of the ray recursion. [default: 50]
";

/// The struct definition for deserializing the data. 
/// 
/// Note: Like the above, this should really be in main.rs instead of here.
/// It is in this file and not in main.rs for no other reason 
/// than that it wouldn't work with my weak Rust programming skills.
#[derive(serde::Deserialize)]
pub struct Args {
    arg_source: String,
    arg_dest: String,
    flag_compute: Option<ComputeEnv>,
    flag_width: isize,
    flag_height: isize,
    flag_samples: isize,
    flag_depth: isize,
}

/// The run function is called from 'main()'. 
/// 
/// It opens up the scene file, creates the necessary memory, 
/// renders the image, and saves to the output file.
/// 
/// It also annotates the errors from the scene parsing and 
/// rendering and propagates them upwards to 'main()' where 
/// they will be handled.
pub fn run(args: &Args) -> anyhow::Result<()> {

    let imgx: u32 = args.flag_width as u32;
    let imgy: u32 = args.flag_height as u32;
    let samples_per_pixel = args.flag_samples as u32;
    let max_depth = args.flag_depth as u32;
    let compute_env = match args.flag_compute {
        Some(s) => s,
        None => ComputeEnv::Multicore
    };

    let file_parser = FileReaderFactory::get_file_processor(&args.arg_source)?;
    let world = file_parser.process_file()?;
    info!("World successfully built!");

    let mut img: RgbImage = ImageBuffer::new(imgx, imgy);
    info!("Output image buffer created.");

    renderer::render(compute_env, samples_per_pixel, max_depth, &world, &mut img)?;

    info!("Saving output to {}", args.arg_dest);
    img.save(&args.arg_dest)?;

    Ok(())
}
