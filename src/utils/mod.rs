use comrak::{markdown_to_html, ComrakOptions};

use std::{fs, io};
pub fn md_to_html(md: &str) -> String {
    markdown_to_html(md, &ComrakOptions::default())
}

pub fn count_md_files() -> io::Result<usize> {
    let paths = fs::read_dir("posts")?;
    return Ok(paths.count())
}