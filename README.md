# rustyrender

`rustyrender` is a simple physically-based monte-carlo CLI ray-tracer written in Rust. It was meant as a learning experience to explore the Rust ecosystem.

## Setup

The project was built using the rust cargo ecosystem, so to build, you can use `$ cargo build` like that Rust boss you know you are. 

## Usage

To run the ray-tracer, simply run `$ rustyrender` with the following usage:

```shell
Usage: rustyrender [options] <source> <dest>
       rustyrender --help

A simple physically based monte-carlo ray tracing renderer written in rust. 

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
```

A sample input file has been supplied at `test_input/one_sphere.json`.

If you would like to use `cargo` to execute the program, it is strongly suggested that you use a release build for performance reasons. You can use the `--release` tag. For example, `$ cargo run --release test_input/one_sphere.json render.png`.


## Testing

To run the unit tests for the project, use `cargo test --lib`. 

The `--lib` flag is necessary because of compile failures in the example documentation.

## Input Formats

The program currently only supports a custom JSON file format to specify the scene file. The schema is the following:
```json
{
    "background_color": {
        "x": 0.5,
        "y": 0.7,
        "z": 1.0
    },
    "camera_config": {
        "vertical_fov": 90.0,
        "look_from": {
            "x": 0.0,
            "y": 0.0,
            "z": 0.0
        },
        "look_at": {
            "x": 0.0,
            "y": 0.0,
            "z": -1.0
        },
        "up": {
            "x": 0.0,
            "y": 1.0,
            "z": 0.0
        },
        "aperture": 0.01,
        "focal_distance": 1.0
    },
    "objects": [
        {
            "type": "Sphere",
            "center": {
                "x": 0.0,
                "y": 0.0,
                "z": -1.0
            },
            "radius": 0.5,
            "material": {
                "lambert": {
                    "albedo": {
                        "x": 0.0,
                        "y": 0.0,
                        "z": 0.5
                    }
                }
            }
        },
        ...
    }
}
```

The background is a color used to render the sky gradient, it represents the color at the zenith. The horizon will be white. There are three types of materials: lambert, metal, and glass. Consult the test_input files for more about the different options.

## Project Organization

There are three logical sections of the code:

- __Executable__: the CLI wrapper around the renderer and parser. This is contained in `src/main.rs` and `src/lib.rs`.
- __Parser__: an API to turn a file into an in-memory representation of the scene. This component is contained in `src/parser`, and the entry point is the `FileReaderFactory` in `src/parser/mod.rs`.
- __Renderer__: an API to render the scene to an image buffer. The entry point for this is the public `render` function in `src/renderer/mod.rs`.

The renderer is broken into three sub-modules:

* __Core__: A module for the core math used by the renderer code. This is under `src/renderer/core`.
* __Scene__: A module containing the objects contained in the scene description and their supporting data structures. This module is under `src/renderer/scene`.
* __Execute__: A module that contains everything needed the execute the ray tracer algorithm. This module is and will be responsible for running the ray-tracer using the different compute backends, like CUDA, OpenCL, and MultiCore, supported on the machine. This module is under `src/renderer/execute`.

## Key Dependencies

This project depends on a number of Rust crates. 

Key to the design of the application are the following crates:

- [thiserror](https://crates.io/crates/thiserror) and [anyhow](https://crates.io/crates/anyhow): Used for error handling ergonomics.
- [serde_json](https://crates.io/crates/serde_json): JSON file parsing
- [crossbeam](https://crates.io/crates/crossbeam): threading and local variable scoping.
- [image](https://crates.io/crates/image): image buffer data structure and image output.