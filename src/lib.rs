#[macro_use]
extern crate comrak;
extern crate tera;
extern crate clap;
extern crate chrono;

pub mod utils;
pub mod cli;

pub static SOURCE_PATH: &'static str          = "source";
pub static SOURCE_POSTS_PATH: &'static str    = "source/_posts";
pub static DIST_POSTS_PATH: &'static str = "posts";
pub static LAYOUTS_POSTS_PATH: &'static str   = "layouts";