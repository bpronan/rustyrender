use super::SceneLoader;

use crate::renderer::scene::world::Region;

use log::{error, info};
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

        // This would be the next major improvement to this library. For a
        // 'real' application, this format would be labored over for at least
        // a week and compared to formats like obj, fbx, collada,
        // and renderman.
        let contents = fs::read_to_string(&self.filename)?;

        let mut world: Region = serde_json::from_str(&contents).map_err(|source| {
            println!("{}", source);
            ParserError::FormatCorrupted { source }
        })?;

        world.recalculate_bounds();

        // World
        Ok(world)
    }
}
