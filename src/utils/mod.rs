use comrak::{markdown_to_html, ComrakOptions};

use std::fs;
use std::io;
use std::io::{ Write, BufReader };
use std::io::prelude::*;

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

pub fn generate_html_files() -> io::Result<()> {
    // println!("{}", SOURCE_POSTS_PATH);
    // for entry in fs::read_dir(SOURCE_POSTS_PATH)? {
    //     // let html_contents = md_to_html(post);
    //     let post = entry.unwrap();
    //     let mut buf_reader = BufReader::new(fs::File::open(post.path())?);
    //     let mut md_contents = String::new();
    //     buf_reader.read_to_string(&mut md_contents)?;

    //     let html_contents = md_to_html(&md_contents);
    //     let full_file_name = post.file_name().into_string().unwrap();
    //     let file_name: Vec<&str> = full_file_name.split(".md").collect();
    //     // println!("{:?}", file_name);
    //     create_html_file(file_name[0], &html_contents);
    // }

    for year_dir in fs::read_dir(SOURCE_POSTS_PATH)? {
        let year_path = year_dir?.path();
        for month_dir in fs::read_dir(year_path)? {
            let month_path = month_dir?.path();
            for day_dir in fs::read_dir(month_path)? {
                let day_path = day_dir?.path();
                for article in fs::read_dir(day_path)? {
                    let post = article.unwrap();
                    let mut buf_reader = BufReader::new(fs::File::open(post.path())?);
                    let mut md_contents = String::new();
                    buf_reader.read_to_string(&mut md_contents)?;

                    let html_contents = md_to_html(&md_contents);
                    let full_file_name = post.file_name().into_string().unwrap();
                    let file_name: Vec<&str> = full_file_name.split(".md").collect();
                    // println!("{:?}", file_name);
                    create_html_file(file_name[0], &html_contents);
                }
            }
        }
    }

    Ok(())
}