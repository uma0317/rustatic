use comrak::{markdown_to_html, ComrakOptions};
use tera::{ Tera, Context };
use tera;

use std::fs;
use std::fs::DirEntry;
use std::io;
use std::io::{ Write, BufReader };
use std::io::prelude::*;
use std::path::{ Path, PathBuf };
use std::cell::RefCell;
use std::rc::Rc;

use statics::{ SOURCE_POSTS_PATH, DIST_POSTS_PATH };
use structs::article::Article;

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
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn generate_html_files() -> io::Result<()> {
    let dist_posts_path = Path::new(DIST_POSTS_PATH);
    let source_posts_path = Path::new(SOURCE_POSTS_PATH);
    
    match functional_generate(&source_posts_path, &dist_posts_path) {
        Ok(_) => {},
        Err(e) => println!("{}", e),
    };
    Ok(())
}

pub fn functional_generate(src_path: &Path, dist_path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src_path)? {
        let entry      = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            let generated_entry_path = dist_path.join(entry.file_name().to_str().unwrap());
            if(!&generated_entry_path.exists()) {
                fs::create_dir(&generated_entry_path);
            }
            functional_generate(&entry_path, &generated_entry_path);
            continue;
        }

        let mut buf_reader  = BufReader::new(fs::File::open(entry_path)?);
        let mut md_contents = String::new();
        buf_reader.read_to_string(&mut md_contents)?;

        let html_contents        = md_to_html(&md_contents);
        let full_file_name       = entry.file_name().into_string().unwrap();
        let file_name: Vec<&str> = full_file_name.split(".md").collect();

        generate_with_layout(dist_path, file_name[0], file_name[0], &html_contents)
    }

    Ok(())
}
 
pub fn render(context: Context) -> tera::Result<()> {
    let mut tera = Tera::new("./source/_layouts/*.html")?;
    let rendered_html = tera.render("article.html", &context)?;

    Ok(())
}

pub fn get_all_articles() -> io::Result<Vec<Article>> {
    let articles: Vec<Article> = vec![];
    let path                   = Path::new(DIST_POSTS_PATH);

    let result = get_all_articles_recursion(path, RefCell::new(articles))?;
    Ok(result.into_inner())
}

pub fn get_all_articles_recursion(dir_path: &Path, articles: RefCell<Vec<Article>>) -> io::Result<RefCell<Vec<Article>>> {
    for entry in fs::read_dir(dir_path)? {
        let entry      = entry?;
        let entry_path = entry.path();

        if (entry.file_name() == ".DS_Store") { continue }
        if entry_path.is_dir() {
            let result_articles = get_all_articles_recursion(&entry_path, articles.clone())?;
            articles.borrow_mut().append(&mut result_articles.into_inner());
        } else {
            let article = Article::new_from_DirEntry(entry);
            articles.borrow_mut().push(article);
        }
    }

    Ok(articles)
}

pub fn create_articles_list() -> io::Result<(String)> {
    let mut tera    = Tera::new("source/_layouts/*.html").unwrap();
    let mut context = Context::new();
    let articles    = get_all_articles()?;

    tera.autoescape_on(vec![]);
    context.add("articles", &articles);

    let rendered_html = tera.render("articles.html", &context).unwrap();
    Ok(rendered_html)
}

pub fn generate_index_html() {
    let rendered_html = create_articles_list().unwrap();
    generate_with_layout(Path::new(""), "index", "index", &rendered_html);
}

fn generate_with_layout(dist_path: &Path, file_name: &str, page_title: &str, contents: &str) {
    let mut tera    = Tera::new("source/_layouts/*.html").unwrap();
    let mut context = Context::new();
    let rendered_header_html = tera.render("header.html", &context).unwrap();

    tera.autoescape_on(vec![]);
    context.add("header", &rendered_header_html);
    context.add("page_title", page_title);
    context.add("contents", contents);

    let rendered_html = tera.render("layout.html", &context).unwrap();
    create_html_file(&dist_path, file_name, &rendered_html);
}