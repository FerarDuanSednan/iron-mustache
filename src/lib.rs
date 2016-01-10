extern crate iron;
extern crate rustache;
extern crate plugin;
extern crate rustc_serialize;

#[macro_use]
extern crate log;

pub use middleware::{MustacheEngine, Template};

mod middleware;
