#[macro_use]
extern crate diesel;
extern crate includedir;
#[macro_use] 
extern crate serde_derive;
extern crate phf;
extern crate tempfile;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate actix;
extern crate actix_web;
extern crate r2d2;
#[macro_use]
extern crate diesel_derive_enum;
extern crate reqwest;
#[macro_use]
extern crate hyper;

pub mod nametag;
pub mod db;
pub mod daemon;
