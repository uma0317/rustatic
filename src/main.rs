extern crate rustatic;
extern crate clap;

use rustatic::{ utils, cli };
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = cli::build().get_matches();

}