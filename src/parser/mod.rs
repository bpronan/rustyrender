mod json;
pub mod error;

use crate::renderer::scene::world::Region;
use crate::parser::error::ParserError;

use json::JSONSceneLoader;

use log::{info, error};
use std::ffi::OsStr;
use std::path::Path;

type BoxResult<T> = Result<T, ParserError>;

pub struct FileReaderFactory;

impl FileReaderFactory {
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
    pub fn get_file_processor(filename: &String) -> BoxResult<Box<dyn SceneLoader>> {

        info!("Opening file {}", filename.to_string());

        let msg; 
        match Path::new(filename)
            .extension()
            .and_then(OsStr::to_str) {
            Some(extension) => {
                match extension {
                    "json" => {
                        return Ok(Box::new(JSONSceneLoader::new(filename)))   
                    },
                    _ => {
                        msg = "Unknown file extension on the input file";
                    }        
                }
            },
            None => {
                msg = "Unknown file extension on the input file";
            }
        }


        error!("{}", msg);
        return Err(ParserError::FileExtensionError);
    }
}

pub trait SceneLoader {
    /// Parses the file and returns an memory scene representation. 
    /// Returns an in memory representation of the scene file.
    fn process_file(&self) -> BoxResult<Region>;
}

