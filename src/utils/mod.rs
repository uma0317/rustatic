use comrak::{markdown_to_html, ComrakOptions};

use std::fs;
use std::io;
use std::io::Write;

use {SOURCE_POSTS_PATH, GENERATED_POSTS_PATH};
pub fn get_config() {

}

pub fn md_to_html(md: &str) -> String {
    markdown_to_html(md, &ComrakOptions::default())
}

pub fn count_md_files() -> io::Result<usize> {
    let paths = fs::read_dir(SOURCE_POSTS_PATH)?;
    return Ok(paths.count())
}

pub fn create_html_file(name: &str, contents: &str) -> io::Result<()>{
    let mut file = fs::File::create(format!("{}/{}.html",GENERATED_POSTS_PATH, name))?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}