#![feature(scoped)] 
#![feature(alloc)] 
#[macro_use]
extern crate log;
mod enviroment;
pub mod server;

pub use self::server::FitnessMessage;

