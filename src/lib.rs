#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use] extern crate diesel;
extern crate dotenv;    

#[macro_use] extern crate diesel_codegen;

pub mod schema;
pub mod model;
pub mod dbmodel;
pub mod persist;
pub mod sync;