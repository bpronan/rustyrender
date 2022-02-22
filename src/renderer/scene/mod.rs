//! The module that contains all of the items contained in the
//! scene.
//!
//! The usage of this module will be with the Region data type
//! in world::Region. This should be opaque to the users of this
//! module; however, to extent the raytracer to draw more objects,
//! or objects to have new materials, this module will need to be
//! extended.

pub mod camera;
pub mod hittable;
pub mod materials;
pub mod objects;
pub mod world;

// a utility for building a procedurally generated world
pub mod world_builder;
