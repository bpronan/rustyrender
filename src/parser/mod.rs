pub mod error;
mod json;

use crate::parser::error::ParserError;
use crate::renderer::scene::world::Region;

use json::JSONSceneLoader;

use log::{error, info};
use std::ffi::OsStr;
use std::path::Path;

type BoxResult<T> = Result<T, ParserError>;

/// Factory method for creating a scene loader based on the file type
/// add a new arm to the extension matching block to add support for
/// different file types.
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
/// use rustyrender::parser::FileReaderFactory;
/// let input_file_path = "test_input/one_sphere.json";
/// let file_parser = FileReaderFactory::get_file_processor(input_file_path).unwrap();
/// let world = file_parser.process_file().unwrap();
/// ```
///
/// # Errors
///
/// * `FileExtensionError` - A file extension that we have yet to add support for.
/// * `SceneCorruptedError` - The scene file doesn't match the schema for scene specification.
/// * `FormatCorruptedError` - The file cannot be parsed by the specific file specification.
/// For example, fbx, json, xml, etc.
/// * `ReadError` - Represents a file read error
/// * `IOError` - Represents any other io error
///
pub struct FileReaderFactory;

impl FileReaderFactory {
    pub fn get_file_processor(filename: &str) -> BoxResult<Box<dyn SceneLoader>> {
        info!("Opening file {}", filename);

        match Path::new(filename).extension().and_then(OsStr::to_str) {
            Some("json") => Ok(Box::new(JSONSceneLoader::new(filename))),
            _ => {
                error!("Unknown file extension on the input file ");
                Err(ParserError::FileExtension)
            }
        }
    }
}

pub trait SceneLoader {
    /// Parses the file and returns an memory scene representation.
    /// Returns an in memory representation of the scene file.
    fn process_file(&self) -> BoxResult<Region>;
}
