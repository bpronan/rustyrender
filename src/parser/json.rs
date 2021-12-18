use super::SceneLoader;

use crate::renderer::core::vector::Point3;
use crate::renderer::scene::world::HittableList;
use crate::renderer::scene::sphere::Sphere;

use log::info;
use serde::{Deserialize, Serialize};
use std::fs;

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
    
        // TODO: Get rid of this clone call
        // TODO: move the read file out of this call. breaks single 
        // responsibility principle.
        let contents = fs::read_to_string(self.filename.clone())?;
        
        let world_object: WorldStruct = serde_json::from_str(&contents)
            .map_err(|source| ParserError::FormatCorrupted{ source })?;
    
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
