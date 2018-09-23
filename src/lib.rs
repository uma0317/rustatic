#[macro_use]
extern crate comrak;
extern crate clap;

pub mod utils;
pub mod generate;
pub mod cli;

pub static SOURCE_POSTS_PATH: &'static str = "source/_posts";
pub static GENERATED_POSTS_PATH: &'static str = "posts";