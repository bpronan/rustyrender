use super::SceneLoader;

use crate::renderer::core::vector::Point3;
use crate::renderer::scene::world::HittableList;
use crate::renderer::scene::objects::sphere::Sphere;

use log::{info, error};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::parser::BoxResult;
use crate::parser::error::ParserError;

pub struct JSONSceneLoader {
    filename: String,
}

impl JSONSceneLoader {
    pub fn new(filenameref: &String) -> JSONSceneLoader {
        JSONSceneLoader { filename: filenameref.clone() }
    }
}


impl SceneLoader for JSONSceneLoader {

    fn process_file(&self) -> BoxResult<HittableList> {
        info!("Parsing world filename {}", self.filename);
        if !Path::new(&self.filename).exists() {
            error!("World input file does not exist at {}", self.filename);
            return Err(ParserError::FileNotFoundError);
        }
    
        #[derive(Debug, Deserialize, Serialize)] struct LocationStruct {
            x: f32,
            y: f32, 
            z: f32,
        }
    
        #[derive(Debug, Deserialize, Serialize)] struct SphereStruct {
            geometry: String,
            center: LocationStruct,
            radius: f32,
        }
    
        #[derive(Debug, Deserialize, Serialize)] struct WorldStruct {
            objects: Vec<SphereStruct>,
        }
    
        let contents = fs::read_to_string(&self.filename)?;
        
        let world_object: WorldStruct = serde_json::from_str(&contents)
            .map_err(|source| ParserError::FormatCorruptedError{ source })?;
    
        // World
        let mut world = HittableList {
            objects: Vec::new(),
        };

        for object in world_object.objects {
            world.objects.push(Box::new(Sphere {
                center: Point3::new(object.center.x, object.center.y, object.center.z),
                radius: object.radius,
            }));    
        }

        Ok(world)
    }
}
