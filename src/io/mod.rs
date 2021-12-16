mod json;
pub mod image;

use crate::renderables::world::HittableList;

use json::JSONSceneLoader;

use log::{info, error};
use std::ffi::OsStr;
use std::path::Path;


/// Interface for all scene file loaders. 
pub trait SceneLoader {
    fn process_file(&self) -> HittableList;
}

pub struct FileReaderFactory;

impl FileReaderFactory {
    /// Factory method for creating a scene loader based on the file type
    /// add a new arm to the extension matching block to add support for 
    /// different file types.
    pub fn get_file_processor(filename: &String) -> Result<Box<dyn SceneLoader>, &'static str> {

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
        return Err(msg)
    }
}