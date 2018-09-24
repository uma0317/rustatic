use comrak::{markdown_to_html, ComrakOptions};
use tera::{ Tera, Context };
use tera;

use std::fs;
use std::fs::DirEntry;
use std::io;
use std::io::{ Write, BufReader };
use std::io::prelude::*;
use std::path::{ Path, PathBuf };
use {SOURCE_POSTS_PATH, DIST_POSTS_PATH};

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

pub fn create_html_file(dist_posts_path: &Path, name: &str, contents: &str) -> io::Result<()>{
    let mut file = fs::File::create(dist_posts_path.join(format!("{}.html", name)))?;
    println!("{:?}" ,dist_posts_path.join(format!("{}.html", name)));
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn generate_html_files() -> io::Result<()> {


    let dist_posts_path = Path::new(DIST_POSTS_PATH);
    let source_posts_path = Path::new(SOURCE_POSTS_PATH);
    println!("{:?}", dist_posts_path);
    match functional(&source_posts_path, &dist_posts_path) {
        Ok(_) => {},
        Err(e) => println!("{}", e),
    };
    Ok(())
}

pub fn functional(src_path: &Path, dist_path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src_path)? {
        let entry                = entry?;
        let entry_path           = entry.path();

        if entry_path.is_dir() {
            let generated_entry_path = dist_path.join(entry.file_name().to_str().unwrap());
            if(!&generated_entry_path.exists()) {
                fs::create_dir(&generated_entry_path);
            }
            functional(&entry_path, &generated_entry_path);
            continue;
        }

        println!("will create");
        let mut tera = Tera::new("./source/_layouts/*.html").unwrap();
        tera.autoescape_on(vec![]);
        let mut context = Context::new();
        context.add("header", &"test");

        let mut buf_reader = BufReader::new(fs::File::open(entry.path())?);
        let mut md_contents = String::new();
        buf_reader.read_to_string(&mut md_contents)?;

        let html_contents = md_to_html(&md_contents);
        let full_file_name = entry.file_name().into_string().unwrap();
        let file_name: Vec<&str> = full_file_name.split(".md").collect();
        // println!("{:?}", file_name);
        context.add("page_title", file_name[0]);
        context.add("contents", &html_contents);
        let rendered_html = tera.render("layout.html", &context).unwrap();

        println!("path: {:?}", src_path);
        create_html_file(dist_path, file_name[0], &rendered_html);
    }

    Ok(())
}
 
pub fn render(context: Context) -> tera::Result<()> {
    let mut tera = Tera::new("./source/_layouts/*.html")?;
    let rendered_html = tera.render("article.html", &context)?;
    println!("{:?}", &rendered_html);

    Ok(())
}