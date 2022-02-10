use super::SceneLoader;

use crate::renderer::core::color::Color;
use crate::renderer::core::vector::Point3;
use crate::renderer::scene::objects::sphere::Sphere;
use crate::renderer::scene::world::Region;

use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::parser::error::ParserError;
use crate::parser::BoxResult;

pub struct JSONSceneLoader {
    filename: String,
}

impl JSONSceneLoader {
    pub fn new(filename: &str) -> JSONSceneLoader {
        JSONSceneLoader {
            filename: filename.to_string(),
        }
    }
}

/// Scene parser for a JSON file.
///
/// This module only supports a json file input.
///
/// # Arguments
///
/// * `filename` - The name of the scene file to parse.
///
/// # Examples
///
/// ```
/// let file_parser = FileReaderFactory::get_file_processor(input_file_path)?;
/// let world = file_parser.process_file()?;
/// ```
///
/// REVIEW: The above example will cause 'cargo test' to fail. I'm struggling
/// getting the module wrangling working within the example code, so in the
/// meantime, run 'cargo test --lib'.
///
/// # Errors
///
/// * `FileExtensionError` - A file extension that we have yet to add support for.
/// * `SceneCorruptedError` - The scene file doesn't match the schema for scene specification.
/// * `FormatCorruptedError` - The file cannot be parsed by the specific file specification.
/// For example, fbx, json, xml, etc.
/// * `ReadError` - Represents a file read error
/// * `IOError` - Represents any other io error
impl SceneLoader for JSONSceneLoader {
    fn process_file(&self) -> BoxResult<Region> {
        info!("Parsing world filename {}", self.filename);
        if !Path::new(&self.filename).exists() {
            error!("World input file does not exist at {}", self.filename);
            return Err(ParserError::FileNotFound);
        }

        // REVIEW: would love to have the parser deserialize the world without
        // this intermediary. Ideally, we could extend this to include other
        // objects without having to change the below code. Currently, that's
        // not the case.
        // This would be the next major improvement to this library. For a
        // 'real' application, this format would be labored over for at least
        // a week and compared to formats like obj, fbx, collada,
        // and renderman.
        #[derive(Debug, Deserialize, Serialize)]
        struct WorldStruct {
            background: Color,
            spheres: Vec<Sphere>,
        }

        let contents = fs::read_to_string(&self.filename)?;

        let world_object: WorldStruct = serde_json::from_str(&contents)
            .map_err(|source| ParserError::FormatCorrupted { source })?;

        // World
        let mut world = Region::new(world_object.background);

        for object in world_object.spheres {
            world.push(Box::new(Sphere {
                center: Point3::new(object.center.x, object.center.y, object.center.z),
                radius: object.radius,
            }));
        }

        Ok(world)
    }
}
