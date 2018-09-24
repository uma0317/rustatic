use utils::*;
use SOURCE_POSTS_PATH;

use std::fs;
use std::io;

use std::io::BufReader;
use std::io::prelude::*;

pub fn generate_html_files() -> io::Result<()> {
    println!("{}", SOURCE_POSTS_PATH);
    for entry in fs::read_dir(SOURCE_POSTS_PATH)? {
        // let html_contents = md_to_html(post);
        let post = entry.unwrap();
        let mut buf_reader = BufReader::new(fs::File::open(post.path())?);
        let mut md_contents = String::new();
        buf_reader.read_to_string(&mut md_contents)?;

        let html_contents = md_to_html(&md_contents);
        let full_file_name = post.file_name().into_string().unwrap();
        let file_name: Vec<&str> = full_file_name.split(".md").collect();
        // println!("{:?}", file_name);
        create_html_file(file_name[0], &html_contents);
    }
    Ok(())
}