use comrak::{markdown_to_html, ComrakOptions};

use std::fs;
use std::io;
use std::io::Write;

use {SOURCE_POSTS_PATH, GENERATED_POSTS_PATH};

pub fn md_to_html(md: &str) -> String {
    markdown_to_html(md, &ComrakOptions::default())
}

pub fn count_md_files() -> io::Result<usize> {
    let mut count = 0;

    for year_dir in fs::read_dir(SOURCE_POSTS_PATH)? {
        let year_path = year_dir?.path();
        for month_dir in fs::read_dir(year_path)? {
            let month_path = month_dir?.path();
            for day_dir in fs::read_dir(month_path)? {
                let day_path = day_dir?.path();
                let day_dir  = fs::read_dir(day_path)?;
                
                count += day_dir.count();
            }
        }
    }

    return Ok(count)
}

pub fn create_html_file(name: &str, contents: &str) -> io::Result<()>{
    let mut file = fs::File::create(format!("{}/{}.html",GENERATED_POSTS_PATH, name))?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}