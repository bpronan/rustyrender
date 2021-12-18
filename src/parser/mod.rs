mod json;
pub mod error;

use crate::renderer::scene::world::HittableList;
use crate::parser::error::ParserError;

use json::JSONSceneLoader;

use log::{info, error};
use std::ffi::OsStr;
use std::path::Path;

type BoxResult<T> = Result<T, ParserError>;

/// Interface for all scene file loaders. 
pub trait SceneLoader {
    fn process_file(&self) -> BoxResult<HittableList>;
}

pub struct FileReaderFactory;

impl FileReaderFactory {
    /// Factory method for creating a scene loader based on the file type
    /// add a new arm to the extension matching block to add support for 
    /// different file types.
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